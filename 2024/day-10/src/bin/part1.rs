use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

const MAX_HIKING_HEIGHT: usize = 9;

struct Grid {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    map: HashMap<Position, usize>
}

impl Grid {
    fn neighbours_of(&self, position: Position) -> HashSet<Position> {
        let mut neighbours = HashSet::new();
        if position.x < self.max_x { neighbours.insert(Position::new(position.x + 1, position.y)); }
        if position.x > self.min_x { neighbours.insert(Position::new(position.x - 1, position.y)); }
        if position.y < self.max_y { neighbours.insert(Position::new(position.x, position.y + 1)); }
        if position.y > self.min_y { neighbours.insert(Position::new(position.x, position.y - 1)); }
        neighbours
    }

    fn ascending_paths(&self, position: Position) -> HashSet<Position> {
        self.neighbours_of(position).iter()
            .filter(|neighbour| *self.map.get(neighbour).unwrap() == *self.map.get(&position).unwrap() + 1)
            .copied()
            .collect()
    }

    fn accessible_trail_ends(&self, position: Position) -> HashSet<Position> {
        if *self.map.get(&position).unwrap() == MAX_HIKING_HEIGHT {
            HashSet::from([position])
        } else {
            self.ascending_paths(position).iter().flat_map(|neighbour| self.accessible_trail_ends(*neighbour)).collect()
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize
}

impl Position {
    fn new(x:usize, y: usize) -> Self {
        Position { x, y }
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let min_x = 0;
    let min_y = 0;

    let mut max_x = 0;
    let mut max_y = 0;

    let mut map = HashMap::new();
    let mut trailheads = HashSet::new();

    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            max_x = i;
            max_y = line.len() - 1;

            line.chars().enumerate()
                .for_each(|(j, val)| { 
                    let height = val.to_digit(10).unwrap() as usize;
                    if height == 0 { trailheads.insert(Position::new(i, j)); }
                    map.insert(Position::new(i, j), height); 
                });
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let grid = Grid { min_x, max_x, min_y, max_y, map };

    let result: usize = trailheads.iter().map(|trailhead| grid.accessible_trail_ends(*trailhead).len()).sum();

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
