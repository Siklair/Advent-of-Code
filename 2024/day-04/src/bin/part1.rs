use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

fn neighbours((a, b): (usize, usize)) -> HashSet<(usize, usize)>{
    let mut neighs = HashSet::new();
    if a > 0 && b > 0{
        neighs.insert((a-1, b-1));
    }
    if a > 0 {
        neighs.insert((a-1, b));
        neighs.insert((a-1, b+1));
    }
    if b > 0 {
        neighs.insert((a, b-1));
        neighs.insert((a+1, b-1));
    }
    neighs.insert((a+1, b));
    neighs.insert((a, b+1));
    neighs.insert((a+1, b+1));
    neighs
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mut pos_map: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    for letter in WORD {
        pos_map.entry(letter).or_default();
    }

    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            for (j, letter) in line.chars().enumerate() {
                if pos_map.contains_key(&letter) {
                    pos_map.get_mut(&letter).unwrap().insert((i, j));
                }
            }
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let mut result: usize = 0;

    // Iterate over all instance of first letter
    for (x, y) in pos_map.get(&WORD[0]).unwrap() {
        // Find all instance of neighbouring second letter
        let neighbours = neighbours((*x, *y));
        let second_neighbour = pos_map.get(&WORD[1]).unwrap().intersection(&neighbours);
        'b: for (x2, y2) in second_neighbour {
            if 2*x2 < *x || 2*y2 < *y {
                continue 'b
            }
            let mut next_pos = (x2 + x2 - x, y2 + y2 - y);
            let mut letter_index = 2;
            'a: while letter_index < WORD.len() {
                // look if the letter exists
                let letter_found = pos_map.get(&WORD[letter_index]).unwrap().contains(&next_pos);
                if !letter_found {
                    break 'a
                }
                // end of loop
                letter_index += 1;
                if next_pos.0 + x2 < *x {
                    break 'a
                }
                next_pos.0 = next_pos.0 + x2 - x;
                if next_pos.1 + y2 < *y {
                    break 'a
                }
                next_pos.1 = next_pos.1 + y2 - y;
            }
            if letter_index == WORD.len() {
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
