use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";
const ITER_NUMBER: usize = 25;

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );
    
    let mut stones: Vec<usize> = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            line.split_whitespace().map(|num| num.parse().unwrap()).for_each(|num| stones.push(num));
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    for _ in 0..ITER_NUMBER {
        let mut new_stones = vec![];

        for stone in stones {
            if stone == 0 {
                new_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let stone_string = stone.to_string();
                new_stones.push(stone_string[..stone_string.len()/2].parse().unwrap());
                new_stones.push(stone_string[stone_string.len()/2..].parse().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }

        stones = new_stones;
    }

    let result: usize = stones.len();

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
