use crate::ResultWorkflow::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Workflow {
    filter: Vec<Box<FilterFn>>,
    default: ResultWorkflow,
}

type FilterFn = dyn FnMut(&Part) -> ResultWorkflow;

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

    fn call(&mut self, part: &Part) -> ResultWorkflow {
        for f in self.filter.iter_mut() {
            match f(part) {
                None => continue,
                x => return x,
            }
        }
        self.default.clone()
    }
}

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

    fn as_fun(&self) -> Box<FilterFn> {
        let (category, ord, val, res) = (self.category, self.ord, self.val, self.res.clone());
        Box::new(move |part| {
            let value = match category {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => unreachable!(),
            };
            if value.cmp(&val) == ord {
                res.clone()
            } else {
                None
            }
        })
    }
}

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

#[derive(Copy, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Self { x, m, a, s }
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
    let mut total = 0;
    let parts = Vec::new();

    for part in parts {
        let mut result = Send("in".to_string());

        while result != Accepted && result != Rejected {
            if let Send(name) = result.clone() {
                let workflow = map_workflows.get_mut(&name).unwrap();
                result = workflow.call(&part);
            } else {
                unreachable!()
            }
        }

        if result == Accepted {
            total += part.x + part.m + part.a + part.s;
        }
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
