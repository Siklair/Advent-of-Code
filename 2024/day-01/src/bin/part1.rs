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

    let mut list1 = vec![];
    let mut list2 = vec![];

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let mut split = line.split_whitespace();
            let (num1, num2) = (split.next().unwrap(), split.next().unwrap());
            list1.push(num1.parse::<usize>().unwrap());
            list2.push(num2.parse::<usize>().unwrap());
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    list1.sort();
    list2.sort();

    let mut result: usize = 0;

    for i in 0..list1.len() {
        result += list1.get(i).unwrap().abs_diff(*list2.get(i).unwrap());
    }

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
