use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use crate::Direction::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

    fn add_direction_nth(&self, direction: Direction, n: usize) -> Self {
        let (dir_x, dir_y) = direction.to_vector();
        Self::new(self.x + dir_x * n as isize, self.y + dir_y * n as isize)
    }

    fn add_direction(&self, direction: Direction) -> Self {
        self.add_direction_nth(direction, 1)
    }

    fn possible_neighbours(&self) -> Vec<Position> {
        vec![self.add_direction(North), self.add_direction(West), self.add_direction(South), self.add_direction(East)]
    }

    fn neighbours(&self, map: &HashMap<Position, char>) -> Vec<Position> {
        let mut neighs = self.possible_neighbours(); 
        neighs.retain(|pos| pos.x >= 0);
        neighs.retain(|pos| *map.get(pos).unwrap() != '#');
        neighs.retain(|pos| {
            match map.get(pos).unwrap() {
                '.' => true,
                '^' => pos.x == self.x - 1,
                '>' => pos.y == self.y + 1,
                '<' => pos.y == self.y - 1,
                'v' => pos.x == self.x + 1,
                _ => unreachable!(),
            }
        });
        neighs
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
    }}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut map = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;

    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.flatten().enumerate() {
            rows = i + 1;
            if rows == 1 { cols = line.len() }
            for (j, c) in line.chars().enumerate() {
                map.insert(Position::new(i as isize, j as isize), c);
            }
        }
    }

    let starting_pos = Position::new(0, (0..cols).find(|j| *map.get(&Position::new(0, *j as isize)).unwrap() == '.').unwrap() as isize);
    let goal = Position::new((rows-1) as isize, (0..cols).find(|j| *map.get(&Position::new((rows-1) as isize, *j as isize)).unwrap() == '.').unwrap() as isize);

    let res = longest_path_from(starting_pos, &map, HashSet::from([starting_pos]), goal) - 1; // I count S in the path

    println!("{res}");
}

fn longest_path_from(pos: Position, map: &HashMap<Position, char>, prevs: HashSet<Position>, goal: Position) -> usize {
    if pos == goal {
        return 1;
    }
    let mut neighs = pos.neighbours(map);
    neighs.retain(|p| !prevs.contains(p));
    let mut res = 0;
    for neigh in neighs {
        let mut prevs_clone = prevs.clone();
        prevs_clone.insert(neigh);
        let local_res = longest_path_from(neigh, map, prevs_clone, goal);
        if local_res > res {
            res = local_res;
        }
    }
    if res == 0 {
        return res;
    }
    res += 1;
    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
