use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new("input.txt")
        );

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {

        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let mut result: usize = 0;

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
