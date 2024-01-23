use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

struct Position {
    num: usize,
    x: usize,
    y: usize,
}

struct Map {
    galaxies: Vec<Position>,
    num: usize, 
    rows: usize,
    col: usize,
}

impl Position {

    fn new(num: usize, x: usize, y: usize) -> Self {
        Self { num, x, y }
    }

    fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

}

impl Map {

    fn new() -> Self {
        Self { galaxies: Vec::new(), num: 0, rows: 0, col: 0 }
    }

    fn add(&mut self, x: usize, y: usize) {
        self.galaxies.push(Position::new(self.num, x, y));
        self.num += 1;
    }

    fn expand(&mut self) {
        for i in (0..self.rows).rev() {
            let mut empty_row = true;
            for pos in self.galaxies.iter() {
                if pos.x == i { empty_row = false; break; }
            }
            if empty_row {
                self.expand_row(i);
            }
        }

        for j in (0..self.col).rev() {
            let mut empty_col = true;
            for pos in self.galaxies.iter() {
                if pos.y == j { empty_col = false; break; }
            }
            if empty_col {
                self.expand_col(j);
            }
        }
    }

    fn expand_row(&mut self, i: usize) {
        self.rows += 1;
        for pos in self.galaxies.iter_mut() {
            if pos.x > i {
                pos.x += 1;
            }
        }
    }

    fn expand_col(&mut self, j: usize) {
        self.col += 1;
        for pos in self.galaxies.iter_mut() {
            if pos.y > j {
                pos.y += 1;
            }
        }
    }

    fn print(&self) {
        let mut map = HashMap::new();
        for i in 0..self.rows {
            for j in 0..self.col {
                map.insert((i, j), '.');
            }
        }
        for galaxy in self.galaxies.iter() {
            map.insert((galaxy.x, galaxy.y), '#');
        }
        for i in 0..self.rows {
            for j in 0..self.col {
                print!("{}", map.get(&(i, j)).unwrap());
            }
            print!("\n");
        }
    }

}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut map = Map::new();

    let mut set_col = false;

    //parsing
    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.enumerate() {
            map.rows += 1; // = i+1

            if let Ok(text) = line {

                if !set_col { set_col = true; map.col = text.len() }

                for (j, c) in text.chars().enumerate() {
                    if c == '#' {
                        map.add(i, j);
                    }
                }
            }
        }
    }

    //computation
    // 1. Expand the universe
    //map.print();
    map.expand();
    //print!("\n\n\n");
    //map.print();

    // 2. Shortest paths
    let mut total = 0;
    for galaxy1 in map.galaxies.iter() {
        for galaxy2 in map.galaxies.iter() {
            if galaxy1.num < galaxy2.num {
                total += galaxy1.distance(galaxy2);
            }
        }
    }

    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
