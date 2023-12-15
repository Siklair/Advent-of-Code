use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const CYCLE_NUMBER: usize = 10;
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input_test.txt")
        );

    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    let mut cols = 0;
    let mut rows = 0;

    // parsing
    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.enumerate() {
            if let Ok(text) = line {
                rows += 1;
                for (j, c) in text.chars().enumerate() {
                    if rows == 1 { cols += 1; }
                    match &c {
                        'O' | '#' => {
                            map.insert((i, j), c);
                        },
                        _ => (), 
                    }
                }

            }
        }
    }

    //computation
    for k in 0..CYCLE_NUMBER {
        for direction in DIRECTIONS {
            for i1 in 0..rows {
                for j1 in 0..cols {
                    let i =
                        if direction.0 == 1 {
                            rows - 1 - i1
                        } else {
                            i1
                        };
                    let j =
                        if direction.1 == 1 {
                            cols - 1 - j1
                        } else {
                            j1
                        };
                    match map.get(&(i, j)) {
                        None | Some('#') => (),
                        Some('O') => roll(&mut map, rows, cols, (i, j), direction),
                        _ => unreachable!(),
                    }
                }
            }
            /* print_map(&map, rows, cols);
            println!(); */
        }
        print_map(&map, rows, cols);
        println!("cycle {}: {}", k, compute_load(&map, rows));
        println!("######");
    }

    let total = compute_load(&map, rows);

    println!("{total}");
}

fn roll(map: &mut HashMap<(usize, usize), char>, rows: usize, cols: usize, (i, j): (usize, usize), direction: (isize, isize)) {
    map.remove(&(i, j));
    let mut current_pos = (i, j);
    loop {
        let future_pos = (current_pos.0 as isize + direction.0, current_pos.1 as isize + direction.1);

        if future_pos.0 < 0 || future_pos.1 < 0 || future_pos.0 >= rows as isize || future_pos.1 >= cols as isize { // out of bonds
            break;
        }

        let next_pos = (future_pos.0 as usize, future_pos.1 as usize);
        match map.get(&next_pos) {
            None => current_pos = next_pos,
            Some('O') | Some('#') => break,
            _ => unreachable!(),
        }
    }
    map.insert(current_pos, 'O');
}

fn compute_load(map: &HashMap<(usize, usize), char>, rows: usize) -> usize {
    let mut res = 0;
    for (i, j) in map.keys() {
        match map.get(&(*i, *j)) {
            Some('#') => (),
            Some('O') => res += rows - i,
            _ => unreachable!()
        }
    }
    res
}

fn print_map(map: &HashMap<(usize, usize), char>, rows: usize, cols: usize) {
    for i in 0..rows {
        for j in 0..cols {
            match map.get(&(i, j)) {
                None => print!("."),
                Some(c) => print!("{c}"),
            }
        }
        println!();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
