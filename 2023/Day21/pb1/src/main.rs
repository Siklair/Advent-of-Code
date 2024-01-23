use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use crate::Direction::*;
use std::ops::Range;
use itertools::Itertools;

const NB_STEPS: usize = 64;

struct GardenMap {
    rows: usize, 
    cols: usize,
    rocks_pos: HashSet<Position>,
}

impl GardenMap {
    fn new() -> Self {
        Self {
            rows: 0,
            cols: 0,
            rocks_pos: HashSet::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
        }
    }

    fn add_direction(&self, direction: Direction) -> Self {
        let (dir_x, dir_y) = direction.to_vector();
        Self::new(self.x + dir_x, self.y + dir_y)
    }   

    fn _origin() -> Self {
        Position::new(0, 0)
    }

    fn neighbours(&self) -> Vec<Position> {
        let mut res = Vec::new();
        for dir in vec![North, West, South, East].into_iter() {
            res.push(self.add_direction(dir));
        }
        res
    }

    fn inbounds(&self, x_range: &Range<isize>, y_range: &Range<isize>) -> bool {
        x_range.contains(&self.x) && y_range.contains(&self.y)
    }

    fn inbound_neighs(&self, x_range: &Range<isize>, y_range: &Range<isize>) -> Vec<Position> {
        self.neighbours().into_iter().filter(|pos| pos.inbounds(x_range, y_range)).collect()
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn to_vector(self) -> (isize, isize) {
        match self {
            North => (-1, 0),
            West => (0, -1),
            South => (1, 0),
            East => (0, 1),
        }
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut map = GardenMap::new();
    let mut rows = 0;
    let mut cols = 0;
    let mut starting_point = Position::new(-1, -1);
    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.flatten().enumerate() {
            rows = i + 1;
            if rows == 1 {
                cols = line.len();
            }

            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    map.rocks_pos.insert(Position::new(i as isize, j as isize));
                } else if c == 'S' {
                    starting_point = Position::new(i as isize, j as isize);
                }
            }
        }
    }
    map.rows = rows;
    map.cols = cols;
    assert!(!starting_point.x < 0 && !starting_point.y < 0);

    let mut current_pos_vec = vec![starting_point];
    // Computation
    for _ in 0..NB_STEPS {            
        current_pos_vec = current_pos_vec
            .clone()
            .into_iter()
            .flat_map(|p| {
                p.inbound_neighs(&(0..rows as isize), &(0..cols as isize))
                .into_iter()
                .filter(|p| map.rocks_pos.get(p) == None)
                .collect::<Vec<_>>()
            })
            .unique()
            .collect();
    }

    for i in 0..map.rows {
        for j in 0..map.cols {
            let pos = &Position::new(i as isize, j as isize);
            let possible_pos: bool = current_pos_vec.contains(pos);
            if map.rocks_pos.contains(pos) {
                print!("#");
                if possible_pos {
                    panic!();
                }
            } else if possible_pos {
                print!("0");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let res = current_pos_vec.len();
    println!("{res}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
