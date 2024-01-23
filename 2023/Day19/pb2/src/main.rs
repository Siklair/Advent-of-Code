use crate::ResultWorkflow::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;
use std::{env, vec};

const SIZE_OF_PART: usize = 4000;

struct Workflow {
    filter: Vec<Box<FilterFn>>,
    default: ResultWorkflow,
}

type FilterFn = dyn FnMut(&PartRange) -> OutputFilter;

impl Workflow {
    fn new(default: ResultWorkflow) -> Self {
        Self {
            filter: Vec::new(),
            default,
        }
    }

    fn add_filter(&mut self, filter: Box<FilterFn>) {
        self.filter.push(filter);
    }

    fn call(&mut self, part: &PartRange) -> OutputWorkflow {
        let mut res = Vec::new();
        let mut to_filter = part.clone();
        let mut last_to_default = true;
        for f in self.filter.iter_mut() {
            let (accepted, rejected) = f(&to_filter);
            if let Some(x) = accepted {
                res.push(x);
            }
            if let Some((part, _)) = rejected {
                to_filter = part;
            } else {
                last_to_default = false;
                break;
            }
        }
        if last_to_default {
            res.push((to_filter, self.default.clone()));
        }
        res
    }
}

#[derive(Clone)]
struct Filter {
    category: char,
    ord: Ordering,
    val: usize,
    res: ResultWorkflow,
}

impl Filter {
    fn new(category: char, ord: Ordering, val: usize, res: ResultWorkflow) -> Self {
        Self {
            category,
            ord,
            val,
            res,
        }
    }

    // returns: (accepted, rejected)
    fn accepted(r: Range<usize>, threshold: usize, ord: Ordering) -> (Range<usize>, Range<usize>) {
        match ord {
            Ordering::Less => (r.start..r.end.min(threshold), r.start.max(threshold)..r.end),
            Ordering::Greater => (
                r.start.max(threshold + 1)..r.end,
                r.start..r.end.min(threshold + 1),
            ),
            _ => unreachable!(),
        }
    }

    fn take_range(&self, part: &PartRange) -> OutputFilter {
        let value = match self.category {
            'x' => &part.x,
            'm' => &part.m,
            'a' => &part.a,
            's' => &part.s,
            _ => unreachable!(),
        };
        let (accepted, rejected) = Self::accepted(value.clone(), self.val, self.ord);
        let accepted_res = if !accepted.is_empty() {
            Some((
                part.clone_but(self.category, accepted.clone()),
                self.res.clone(),
            ))
        } else {
            Option::None
        };
        let rejected_res = if !rejected.is_empty() {
            Some((part.clone_but(self.category, rejected.clone()), None))
        } else {
            Option::None
        };
        /* {
            println!(
                "value: {value:?}, threshold: {val}, ord: {ord:?}, accepted: {accepted:?}, rejected: {rejected:?}",
                val = self.val,
                ord = self.ord,
            );
        } */
        (accepted_res, rejected_res)
    }

    fn as_fun(&self) -> Box<FilterFn> {
        let clone = self.clone();
        Box::new(move |part| clone.take_range(part))
    }
}

type Pairing = (PartRange, ResultWorkflow);
type OutputFilter = (Option<Pairing>, Option<Pairing>);
type OutputWorkflow = Vec<Pairing>;

#[derive(Clone, Eq, PartialEq)]
enum ResultWorkflow {
    Accepted,
    Rejected,
    Send(String), // workflow name
    None,
}

impl ResultWorkflow {
    fn from_str(s: &str) -> Self {
        match s {
            "A" => Accepted,
            "R" => Rejected,
            x => Send(x.to_string()),
        }
    }
}

#[derive(Clone)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRange {
    fn new(x: Range<usize>, m: Range<usize>, a: Range<usize>, s: Range<usize>) -> Self {
        Self { x, m, a, s }
    }

    fn clone_but(&self, category: char, range: Range<usize>) -> Self {
        match category {
            'x' => Self::new(range, self.m.clone(), self.a.clone(), self.s.clone()),
            'm' => Self::new(self.x.clone(), range, self.a.clone(), self.s.clone()),
            'a' => Self::new(self.x.clone(), self.m.clone(), range, self.s.clone()),
            's' => Self::new(self.x.clone(), self.m.clone(), self.a.clone(), range),
            _ => unreachable!(),
        }
    }

    fn len(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

fn main() {
    let file = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join(Path::new("input.txt"));

    let mut map_workflows: HashMap<String, Workflow> = HashMap::new();

    // parsing
    let mut at_part: bool = false;
    if let Ok(lines) = read_lines(file) {
        for text in lines.flatten() {
            if text.is_empty() {
                at_part = true;
                continue;
            }

            if !at_part {
                // parse px{a<2006:qkq,m>2090:A,rfg}
                let (name, remainder) = text.split_once('{').unwrap();

                let mut filters: Vec<Filter> = Vec::new();
                let default;

                let mut split = remainder.split(',');
                loop {
                    let s = split.next().unwrap();
                    if s.contains(':') {
                        let category = s[0..1].to_string().clone().chars().next().unwrap();
                        let comp = match s[1..2].to_string().clone().chars().next().unwrap() {
                            '<' => Ordering::Less,
                            '>' => Ordering::Greater,
                            _ => unreachable!(),
                        };

                        let mut split = s.split(':');
                        let val: usize = split.next().unwrap()[2..].parse().unwrap();
                        let result = ResultWorkflow::from_str(split.next().unwrap());

                        filters.push(Filter::new(category, comp, val, result));
                    } else {
                        default = ResultWorkflow::from_str(&s[..(s.len() - 1)]);
                        break;
                    }
                }

                let mut workflow = Workflow::new(default);
                for filter in filters {
                    workflow.add_filter(filter.as_fun());
                }

                map_workflows.insert(name.to_string(), workflow);
            }
        }
    }

    // computation

    // ranges from 1 to SIZE_OF_PART
    let init_range = 1..SIZE_OF_PART + 1;
    let init_part = PartRange::new(
        init_range.clone(),
        init_range.clone(),
        init_range.clone(),
        init_range.clone(),
    );
    let result = Send("in".to_string());
    let mut parts = vec![(init_part, result)];
    let mut total = 0;

    while !parts.is_empty() {
        let mut new_parts = Vec::new();
        for (part, result) in parts {
            match result {
                Accepted => {
                    total += part.len();
                }
                Rejected => {}
                Send(name) => {
                    let workflow = map_workflows.get_mut(&name).unwrap();
                    let mut res = workflow.call(&part);
                    new_parts.append(&mut res);
                }
                None => unreachable!(),
            }
        }
        parts = new_parts;
    }

    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
