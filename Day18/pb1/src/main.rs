use std::collections::{HashMap, HashSet};
use std::env;
use std::ops::Range;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use crate::Direction::*;

struct Map {
    x_range: Range<isize>,
    y_range: Range<isize>,
    elts: HashMap<Position, char>,
}

impl Map {
    fn new() -> Self {
        Self { 
            x_range: 0..0, 
            y_range: 0..0, 
            elts: HashMap::new(),
        }
    }

    fn add(&mut self, pos: Position) -> Option<char> {
        self.x_range = Self::extend_range(self.x_range.clone(), pos.x);
        self.y_range = Self::extend_range(self.y_range.clone(), pos.y);
        self.elts.insert(pos, '#')
    }

    fn add_range(&mut self, pos: Position, direction: Direction, n: usize) -> Position { // Returns the last element added
        let mut next_pos = pos;
        for _ in 0..n {
            next_pos = next_pos.add_direction(direction);
            self.add(next_pos);
        }
        next_pos
    }

    fn extend_range(r: Range<isize>, n: isize) -> Range<isize> {
        if n >= r.end {
            r.start..n+1
        } else if n < r.start {
            n..r.end
        } else {
            r
        }
    }

    fn print(&self) {
        for i in self.x_range.clone() {
            for j in self.y_range.clone() {
                match self.elts.get(&Position::new(i, j)) {
                    None => print!("."),
                    Some('#') => print!("#"),
                    _ => unreachable!(),
                }
            }
            println!();
        }
    } 


    // Adapted from Day10 part 2
    // Let's add a row and column on each side of the map and flood fill the exterior, we get: interior = map - #s - exterior
    // returns: all points in the map that are outside the lagoon
    fn flood_fill_ext(&self, starting_point: Position) -> HashSet<Position> {
        let mut outside_points: HashSet<Position> = HashSet::new();
        let mut to_explore = HashSet::from([starting_point]);
        let mut to_be_explored = HashSet::new();

        let new_x_range = self.x_range.start-1..self.x_range.end+1;
        let new_y_range = self.y_range.start-1..self.y_range.end+1;

        while !to_explore.is_empty() {
            for point in to_explore.iter() {
                for neigh in point.inbound_neighs(&new_x_range, &new_y_range) { // get neighbours in all directions
                    if !(self.elts.contains_key(&neigh) || outside_points.contains(&neigh) || to_explore.contains(&neigh) || to_be_explored.contains(&neigh)) {
                        to_be_explored.insert(neigh);
                    }
                }
                outside_points.insert(*point);
            }
            to_explore = to_be_explored.clone();
            to_be_explored.clear();
        }
        outside_points.into_iter().filter(|pos| pos.inbounds(&self.x_range, &self.y_range)).collect()
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

    fn origin() -> Self {
        Position::new(0, 0)
    }

    fn neighbours(&self) -> Vec<Position> {
        let mut res = Vec::new();
        for dir in vec![NORTH, WEST, SOUTH, EAST].into_iter() {
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
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

impl Direction {
    fn to_vector(&self) -> (isize, isize) {
        match self {
            NORTH => (-1, 0),
            WEST => (0, -1),
            SOUTH => (1, 0),
            EAST => (0, 1),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'U' => NORTH,
            'L' => WEST,
            'D' => SOUTH,
            'R' => EAST,
            _ => unreachable!(),
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
    let mut current_pos = Position::origin();
    map.add(current_pos);

    //parsing
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {
                let mut split = text.split(" ");
                let direction = Direction::from_char(split.next().unwrap().chars().next().unwrap());
                let num: usize = split.next().unwrap().parse().unwrap();
                let _color = split.next().unwrap();

                current_pos = map.add_range(current_pos, direction, num);
            }
        }
    }

    //map.print();
    //println!("{:?} // {:?}", map.x_range, map.y_range);

    //computation
    let starting_point = Position::new(map.x_range.start-1, map.y_range.start-1);
    let outside_points = map.flood_fill_ext(starting_point);

    println!("flood fill ended");

    let mut interior_points = Vec::new();
    for i in map.x_range.clone() {
        for j in map.y_range.clone() {
            let pos = Position::new(i, j);
            if !map.elts.contains_key(&pos) && !outside_points.contains(&pos) {
                interior_points.push(pos);
            }
        }
    }

    for pos in interior_points {
        map.add(pos);
    }

    //map.print();

    let res = map.elts.keys().len();
    println!("{res}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
