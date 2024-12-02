use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

fn index_element(map: &mut HashMap<usize, (usize, usize)>, elem1: usize, elem2: usize) {
    map.entry(elem1).or_insert((0, 0)).0 += 1;
    map.entry(elem2).or_insert((0, 0)).1 += 1;
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mut map = HashMap::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let mut split = line.split_whitespace();
            let (num1, num2) = (split.next().unwrap(), split.next().unwrap());
            index_element(&mut map, num1.parse().unwrap(), num2.parse().unwrap());
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }


    let mut result: usize = 0;

    for key in map.keys() {
        let (a, b) = map.get(key).unwrap();
        result += key * a * b; 
    }

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
