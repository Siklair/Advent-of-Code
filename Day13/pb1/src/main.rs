use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

// returns the number of lines left to the symmetry axis
fn find_vertical_symmetry(map: &HashMap<(usize, usize), char>, rows: usize, cols: usize) -> Option<usize> {
    for j1 in 0..cols-1 {
        let j2 = j1 + 1;
        // is j1/j2 a symmetry axis ?
        if test_symmetry_vertical(map, j1, j2, rows, cols) {
            //println!("j2: {j2}");
            //println!("{:?}", map);
            return Some(j2);
        }
    }
    None
}

fn test_symmetry_vertical(map: &HashMap<(usize, usize), char>, j1: usize, j2: usize, rows: usize, cols: usize) -> bool {
    /* println!("j1: {j1} \n j2: {j2} \n cols: {cols} \n ");

    println!("##########");
    println!("{:?}", map);
    println!("##########"); */

    if j1 == 0 || j2 == cols-1 {
        return vertical_equal(map, j1, j2, rows);
    }
    vertical_equal(map, j1, j2, rows) && test_symmetry_vertical(map, j1-1, j2+1, rows, cols)
}

fn vertical_equal(map: &HashMap<(usize, usize), char>, j1: usize, j2: usize, rows: usize) -> bool {
    for i in 0..rows {
        if map.get(&(i, j1)).unwrap() != map.get(&(i, j2)).unwrap() {
            return false;
        }
    }
    true
}

// returns the number of lines above the symmetry axis
fn find_horizontal_symmetry(map: &HashMap<(usize, usize), char>, rows: usize, cols: usize) -> Option<usize> {
    //println!("rows: {rows} \n cols: {cols} \n");
    for i1 in 0..rows-1 {
        let i2 = i1 + 1;
        // is i1/i2 a symmetry axis ?
        if test_symmetry_horizontal(map, i1, i2, rows, cols) {
            //println!("i2: {i2}");
            return Some(i2);
        }
    }
    None
}

fn test_symmetry_horizontal(map: &HashMap<(usize, usize), char>, i1: usize, i2: usize, rows: usize, cols: usize) -> bool {
    if i1 == 0 || i2 == rows-1 {
        return horizontal_equal(map, i1, i2, cols);
    }
    horizontal_equal(map, i1, i2, cols) && test_symmetry_horizontal(map, i1-1, i2+1, rows, cols)
}

fn horizontal_equal(map: &HashMap<(usize, usize), char>, i1: usize, i2: usize, cols: usize) -> bool {
    for j in 0..cols {
        if map.get(&(i1, j)).unwrap() != map.get(&(i2, j)).unwrap() {
            return false;
        }
    }
    true
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    let mut cols = 0;
    let mut rows = 0;

    let mut vertical_res = 0;
    let mut horizontal_res = 0;
    let mut i = 0;

    // parsing
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {

                if text.is_empty() {
                    // computation
                    match find_vertical_symmetry(&map, rows, cols) {
                        Some(a) => vertical_res += a,
                        None => match find_horizontal_symmetry(&map, rows, cols) {
                            Some(b) => horizontal_res += b,
                            None => unreachable!(),
                        }
                    }
                    // clean up
                    map.clear();
                    cols = 0;
                    rows = 0;
                    i = 0;
                } else {
                    for (j, c) in text.chars().enumerate() {
                        map.insert((i, j), c);
                        if i == 0 {
                            cols += 1;
                        }
                    }
                    rows += 1;
                    i += 1;
                }

            }
        }
    }

    let total = vertical_res + 100 * horizontal_res;

    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
