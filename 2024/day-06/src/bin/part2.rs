use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up
        }
    }

    fn going_out_of_bonds(&self, position: (usize, usize), max_lines: usize, max_cols: usize) -> bool {
        match self {
            Direction::Up => position.0 == 0,
            Direction::Right => position.1 == max_cols,
            Direction::Down => position.0 == max_lines,
            Direction::Left => position.1 == 0
        }
    }

    // Returns empty optional if position becomes negative
    fn next_position(&self, position: (usize, usize), max_lines: usize, max_cols: usize) -> Option<(usize, usize)> {
        if self.going_out_of_bonds(position, max_lines, max_cols) {
            return None;
        }
        let mut next_pos = position;
        match self {
            Direction::Up => next_pos.0 -= 1,
            Direction::Right => next_pos.1 += 1,
            Direction::Down => next_pos.0 += 1,
            Direction::Left => next_pos.1 -= 1
        }
        Some(next_pos)
    }
}

// Returns empty optional if out of the map
fn guard_move(position: (usize, usize), direction: Direction, obstacles: &HashSet<(usize, usize)>, max_lines: usize, max_cols: usize) 
    -> (Option<(usize, usize)>, Direction){
        if let Some(next_position) = direction.next_position(position, max_lines, max_cols) {
            if obstacles.contains(&next_position) {
                (Some(position), direction.turn_right())
            } else {
                (Some(next_position), direction)
            }
        } else {
            (None, direction)
        }
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mut max_lines = 0;
    let mut max_cols = 0;

    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    let mut guard_pos: Option<(usize, usize)> = None;

    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            max_lines = i;
            max_cols = line.len()-1;
            line.match_indices('#').map(|(k, _)| k).for_each(|k| { obstacles.insert((i, k)); });
            if line.contains('^') {
                guard_pos = Some((i, line.find('^').unwrap()));
            }
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let mut visited_pos = HashSet::new();
    let mut direction = Direction::Up;
    
    while let Some(position) = guard_pos {
        visited_pos.insert(position);
        (guard_pos, direction) = guard_move(position, direction, &obstacles, max_lines, max_cols);
    }

    let result = visited_pos.len();

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
