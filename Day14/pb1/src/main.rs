use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

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
    for i in 0..rows {
        for j in 0..cols {
            match map.get(&(i, j)) {
                None | Some('#') => (),
                Some('O') => roll_north(&mut map, (i, j)),
                _ => unreachable!(),
            }
        }
    }

    //print_map(&map, rows, cols);

    let total = compute_load(&mut map, rows);

    println!("{total}");
}

fn roll_north(map: &mut HashMap<(usize, usize), char>, (i, j): (usize, usize)) {
    map.remove(&(i, j));
    let mut current_pos = (i, j);
    while current_pos.0 > 0 {
        match map.get(&(current_pos.0-1, current_pos.1)) {
            None => current_pos.0 -= 1,
            Some('O') | Some('#') => break,
            _ => unreachable!(),
        }
    }
    map.insert(current_pos, 'O');
}

fn compute_load(map: &mut HashMap<(usize, usize), char>, rows: usize) -> usize {
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
