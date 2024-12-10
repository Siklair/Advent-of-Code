use std::collections::{HashMap, HashSet};
use std::env;
use std::ops::{Add, AddAssign, Neg, Sub};
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vector {
    x: isize,
    y: isize
}

impl Vector {
    fn from_unsigned((i, j): (usize, usize)) -> Self {
        Vector { x: i.try_into().unwrap(), y: j.try_into().unwrap() }
    }

    fn antinodes(&self, other: Vector, x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> HashSet<Vector> {
        let mut antinodes = HashSet::new();

        let mut direction = self.vector_to(other);
        let mut next_node = other;
        while next_node.within_bounds(x_min, x_max, y_min, y_max) {
            antinodes.insert(next_node);
            next_node += direction;
        }
        direction = - direction;
        next_node = *self;
        while next_node.within_bounds(x_min, x_max, y_min, y_max) {
            antinodes.insert(next_node);
            next_node += direction;
        }
        antinodes
    }

    fn vector_to(&self, other: Vector) -> Vector {
        other - *self
    }

    fn within_bounds(&self, x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> bool {
        (x_min..=x_max).contains(&self.x) && (y_min..=y_max).contains(&self.y)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Vector::from_unsigned((0, 0)) - self
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mut antennas_position: HashMap<char, Vec<Vector>> = HashMap::new();

    let mut max_lines = 0;
    let mut max_cols = 0;

    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas_position.entry(c).or_default().push(Vector::from_unsigned((i, j)));
                }
                max_lines = i;
                max_cols = j;
            }
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let mut antinodes = HashSet::new();

    for key in antennas_position.keys() {
        let antennas = antennas_position.get(key).unwrap();
        for a in antennas {
            for b in antennas {
                if a != b {
                    let nodes = a.antinodes(*b, 0, max_lines as isize, 0, max_cols as isize);
                    for node in nodes {
                        antinodes.insert(node);
                    }
                }
            }
        }
    }

    let result: usize = antinodes.len();

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
