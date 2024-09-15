use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Ordering::{Less, Greater};

const INPUT_FILE_NAME: &str = "input.txt";
const THRESHOLD: usize = 2;

fn mark_position(map: &mut HashMap<(usize, usize), usize>, (x, y): (usize, usize), counter: &mut usize) {
    let count = map.entry((x, y)).or_insert(0);
    *count += 1;
    if *count == THRESHOLD {
        *counter += 1;
    } 
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );
    
    let mut count_vents: HashMap<(usize, usize), usize> = HashMap::new();
    let mut count_over_threshold: usize = 0;

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let (p1, p2) = line.split_once(" -> ").unwrap();
            let (x1, y1) = p1.split_once(',').unwrap();
            let (x2_str, y2_str) = p2.split_once(',').unwrap();
            let (x2, y2): (usize, usize) =  (x2_str.parse().unwrap(), y2_str.parse().unwrap());
            let (mut x, mut y): (usize, usize) = (x1.parse().unwrap(), y1.parse().unwrap());
            while (x, y) != (x2, y2) {
                mark_position(&mut count_vents, (x, y), &mut count_over_threshold);
                match x.cmp(&x2) {
                    Less => x += 1,
                    Greater => x -= 1,
                    _ => ()
                }
                match y.cmp(&y2) {
                    Less => y += 1,
                    Greater => y -= 1,
                    _ => ()
                }
            }
            assert!(x == x2 && y == y2);
            mark_position(&mut count_vents, (x, y), &mut count_over_threshold);
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let result: usize = count_over_threshold;

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
