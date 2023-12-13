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

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {

            }
        }
    }

    println!("no result");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
