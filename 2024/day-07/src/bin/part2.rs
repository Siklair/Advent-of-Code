use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use rayon::prelude::*;

const INPUT_FILE_NAME: &str = "input.txt";
const OPERATORS: [Operator; 3] = [Operator::Add, Operator::Mul, Operator::Concat];

enum Operator {
    Add,
    Mul, 
    Concat
}

impl Operator {
    fn inverse(&self, a: isize, b: isize) -> Option<isize> {
        match self {
            Self::Add => Some(a - b),
            Self::Mul => if a % b == 0 {
                Some (a / b)
            } else {
                None
            },
            // strip suffix because numbers are taken in reverse order to factorize
            Self:: Concat => a.to_string().strip_suffix(&b.to_string()).map(|n| n.parse().unwrap())
        }
    }
}

// values: values to combine to get target
fn feasability(target: isize, values: &[isize]) -> bool {
    let mut feasable = false;
    if values.is_empty() {
        unreachable!("Values is empty")
    } else if values.len() == 1 {
        feasable = *values.first().unwrap() == target;
    } else {
        let first = values.first().unwrap();
        for op in OPERATORS {
            if let Some(next_target) = op.inverse(target, *first) {
                if feasability(next_target, &values[1..]) {
                    feasable = true;
                    break;
                }
            }
        }
    }
    feasable
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mut inputs: Vec<(isize, Vec<isize>)> = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let (target, values) = line.split_once(": ").unwrap();
            let target_number: isize = target.parse().unwrap();
            let mut values_number: Vec<isize> = values.split_whitespace().map(|n| n.parse().unwrap()).collect();
            values_number.reverse(); // reverse because operations are processed left to right but solver uses factorization
            inputs.push((target_number, values_number));
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }
    
    let now = Instant::now();

    let result: isize = inputs.par_iter().map(|(target, values)|
        if feasability(*target, values) {
            target
        } else {
            &0
        }
    ).sum();


    let elapsed = now.elapsed();

    println!("Result: {}, elapsed time: {:?}", result, elapsed);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
