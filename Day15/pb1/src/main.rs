use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut total = 0;

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {

                if text.contains('\n') { unreachable!() }
                let mut split = text.split(",");

                for s in split {
                    total += hash_algorithm(s);
                }

            }
        }
    }

    println!("{total}");
}

fn hash_algorithm(s: &str) -> usize {
    let mut current_value = 0;
    for c in s.chars() {
        current_value += c as usize; // ascii representation of c
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
