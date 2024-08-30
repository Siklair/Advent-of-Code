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

    let mut measurements: Vec<u16> = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let depth: u16 = line.parse().unwrap();
            measurements.push(depth);
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let mut result = 0;
    let mut last_measurement = *measurements.first().unwrap();
    for &current_measurement in &measurements[1..measurements.len()] {
        if current_measurement > last_measurement {
            result += 1;
        }
        last_measurement = current_measurement;
    }

    println!("result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
