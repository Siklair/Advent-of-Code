use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";
const BINARY_SIZE: usize = 12;

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new("input.txt")
        );

        // (n, m) where n is the number of 0 bits, m the number of 1 bits
        let mut bits_count = [(0, 0); BINARY_SIZE];

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            assert!(line.len() == BINARY_SIZE);
            for (i, c) in line.char_indices() {
                match c {
                    '0' => bits_count[i].0 += 1,
                    '1' => bits_count[i].1 += 1,
                    _ => unreachable!()
                }
            }
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let mut gamma = 0;
    for pair in bits_count.iter() {
        gamma <<= 1;
        if pair.1 > pair.0 {
            gamma += 1;
        }
    }
    let epsilon = u64::pow(2, BINARY_SIZE as u32) -1 - gamma;

    println!("Epsilon {epsilon} and gamma {gamma}");

    let result: u64 = gamma * epsilon;

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
