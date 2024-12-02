use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

const MIN_DIFF: usize = 1;
const MAX_DIFF: usize = 3;

fn is_safe(report: Vec<usize>) -> bool {
    let mut order_set = HashSet::new();
    for nums in report.windows(2) {
        let a = nums[0];
        let b = nums[1];
        let distance = a.abs_diff(b);
        if !(MIN_DIFF..=MAX_DIFF).contains(&distance) {
            return false;
        }
        let ordering = a.cmp(&b);
        if ordering != Ordering::Equal {
            order_set.insert(ordering);
            if order_set.len() > 1 {
                return false;
            }
        }
    }
    true
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
