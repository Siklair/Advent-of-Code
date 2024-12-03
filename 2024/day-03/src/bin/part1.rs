use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mul_regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let mut result: usize = 0;

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let muls: Vec<&str> = mul_regex.find_iter(&line).map(|m| m.as_str()).collect();
            for mul in muls {
                let (a, b) = mul.split(&['(', ')']).nth(1).unwrap().split_once(',').unwrap();
                result += a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap();
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
