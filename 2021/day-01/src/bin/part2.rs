use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";
const SLIDING_WINDOW_SIZE: usize = 3;

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
    let mut windows_iter = measurements.windows(SLIDING_WINDOW_SIZE);
    let mut last_window = windows_iter.next().unwrap();
    for current_window in windows_iter {
        if current_window.iter().sum::<u16>() > last_window.iter().sum::<u16>() {
            result += 1;
        }
        last_window = current_window;
    }

    println!("result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
