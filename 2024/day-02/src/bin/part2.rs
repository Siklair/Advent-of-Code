use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

const MIN_DIFF: usize = 1;
const MAX_DIFF: usize = 3;

pub enum Intersection {
    All,
    Nums(HashSet<usize>)
}

impl Intersection {
    fn intersect(&mut self, other: (usize, usize)) {
        let others = HashSet::from([other.0, other.1]);
        match self {
            Self::All => *self = Self::Nums(HashSet::from([other.0, other.1])),
            Self::Nums(nums) => *self = Self::Nums(nums.intersection(&others).cloned().collect())
        }
    }

    fn is_empty(&self) -> bool {
        if let Intersection::Nums(nums) = self {
            if nums.is_empty() {
                return true;
            }
        }
        false
    }
}

fn is_safe(report: Vec<usize>) -> bool {
    let mut global_ordering: Option<Ordering> = Option::None;
    let mut collected_orderings: HashMap<Ordering, (usize, usize)> = HashMap::new();
    let mut error_intersect = Intersection::All;
    // iterate over windows 
    for (i, nums) in report.windows(2).enumerate() {
        let j = i + 1; // the second index of the window
        let mut in_error = HashSet::new();
        let a = nums[0];
        let b = nums[1];
    // // if dist not in diff_range => collect as error
        if !(MIN_DIFF..=MAX_DIFF).contains(&a.abs_diff(b)) {
            in_error.insert((i, j));
        }
    // // ### BEGIN: ORDERING ###
        let current_ordering = a.cmp(&b);
        if current_ordering != Ordering::Equal {
        // // collect ordering if global ordering not defined
            if global_ordering.is_none() {
                if let Entry::Vacant(e) = collected_orderings.entry(current_ordering) {
                    e.insert((i, j));
                } else {
        // // define global ordering if ordering count >= 2
                    global_ordering = Option::Some(current_ordering);
        // // if global ordering just defined => collect previous different ordering as error
                    if collected_orderings.contains_key(&current_ordering.reverse()) {
                        let previous_error = *collected_orderings.get(&current_ordering.reverse()).unwrap();
                        in_error.insert(previous_error);
                    }
                }
            } else if current_ordering == global_ordering.unwrap().reverse() {
        // // if ordering different from general ordering => collect as error
                in_error.insert((i, j));
            }
        }
    // // ### END: ORDERING ###
    // // compute collected_errors intersection
        for (x, y) in in_error {
            error_intersect.intersect((x, y));
        }
    // // if new collected error not intersect collected_errors => return not safe
        if error_intersect.is_empty() {
            return false;
        }
    }
    // At this point we have all indices possibly in error
    match error_intersect {
        // No errors encountered
        Intersection::All => true,
        Intersection::Nums(nums) => {
            // iterate over possible errors
            for index in nums {
                if index == 0 || index == report.len()-1 {
                    return true;
                } else {
                    let (prev, next) = (report.get(index-1).unwrap(), report.get(index+1).unwrap());
                    if (MIN_DIFF..=MAX_DIFF).contains(&prev.abs_diff(*next)) && prev.cmp(next) == global_ordering.unwrap() {
                        return true;
                    }
                }
            }
            false
        }
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

        let mut result: usize = 0;

        if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let report = line.split(' ').map(|val| val.parse().unwrap()).collect();
            if is_safe(report) {
                result += 1;
            }
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
