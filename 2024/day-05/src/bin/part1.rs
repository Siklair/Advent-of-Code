use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mut required_map: HashMap<usize,HashSet<usize>> = HashMap::new();

    let mut result: usize = 0;

    if let Ok(lines) = read_lines(file) {
        let mut second_part = false;
        for line in lines.map_while(Result::ok) {
            if !second_part {
                if line.is_empty() {
                    second_part = true;
                } else {
                    let nums_str = line.split_once('|').unwrap();
                    required_map.entry(nums_str.1.parse().unwrap()).or_default().insert(nums_str.0.parse().unwrap());
                }
            } else {
                let mut visited_nums: Vec<usize> = Vec::new();
                let mut ordering_error = false;
                'a: for num_str in line.split(',') {
                    for visited in &visited_nums {
                        if required_map.contains_key(visited) && required_map.get(visited).unwrap().contains(&num_str.parse().unwrap()) {
                            ordering_error = true;
                            break 'a;
                        }
                    }
                    visited_nums.push(num_str.parse().unwrap());
                }
                if !ordering_error {
                    result += visited_nums.get(visited_nums.len() / 2).unwrap();
                }
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
