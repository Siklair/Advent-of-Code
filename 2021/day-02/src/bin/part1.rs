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

    let mut total_horizontal_movement = 0;
    let mut depth = 0;

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let (direction, distance_str) = line.split_once(" ").unwrap();
            let distance: i16 = distance_str.parse().unwrap();
            match direction {
                "forward" => total_horizontal_movement += distance,
                "down" => depth += distance,
                "up" => depth -= distance,
                _ => unreachable!("The input is unknown")
            }
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let result: isize = depth as isize * total_horizontal_movement as isize;

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
