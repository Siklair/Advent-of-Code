use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

const INPUT_FILE_NAME: &str = "input.txt";

fn match_indices(input: &String, patterns: HashSet<String>) -> Vec<(String, usize)> {
    let mut matches = vec![];
    for i in 0..input.len() {
        for pattern in &patterns {
            if input.get(i..i+pattern.len()).unwrap_or("") == pattern {
                matches.push((pattern.to_string(), i));
            }
        }
    }
    matches
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mul_regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let mut result: usize = 0;

    if let Ok(lines) = read_lines(file) {
        let mut in_do = true;
        for line in lines.map_while(Result::ok) {
            println!("line");

            let matches = match_indices(&line, HashSet::from(["do()".to_string(), "don't()".to_string()]));

            // BEGIN
            let mut last_index = 0;
            for (val, index) in matches {
                if in_do && val == "don't()" {
                    //println!("{}", line.get(last_index..index).unwrap());
                    let muls: Vec<&str> = mul_regex.find_iter(line.get(last_index..index).unwrap_or("")).map(|m| m.as_str()).collect();
                    for mul in muls {
                        let (a, b) = mul.split(&['(', ')']).nth(1).unwrap().split_once(',').unwrap();
                        //println!("{result} += mul({}, {})", a, b);
                        result += a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap();
                    }
                    last_index = index;
                    in_do = false;
                } else if !in_do && val == "do()" {
                    last_index = index;
                    in_do = true;
                }
            }
            if in_do {
                //println!("{}", line.get(last_index..line.len()).unwrap());
                let muls: Vec<&str> = mul_regex.find_iter(line.get(last_index..line.len()).unwrap_or("")).map(|m| m.as_str()).collect();
                    for mul in muls {
                        let (a, b) = mul.split(&['(', ')']).nth(1).unwrap().split_once(',').unwrap();
                        //println!("{result} += mul({}, {})", a, b);
                        result += a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap();
                    }
            }
            // END
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
