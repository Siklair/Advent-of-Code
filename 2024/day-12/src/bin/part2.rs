use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Position {
    x: usize,
    y: usize
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut neighbours = Vec::new(); 
        if self.x > 0 {
            neighbours.push(Position::new(self.x - 1, self.y));
        }
        if self.y > 0 {
            neighbours.push(Position::new(self.x, self.y - 1));
        }
        neighbours.push(Position::new(self.x + 1, self.y));
        neighbours.push(Position::new(self.x, self.y + 1));
        neighbours
    }

    fn diags(&self) -> Vec<Self> {
        vec![
            Position::new(self.x - 1, self.y - 1),
            Position::new(self.x - 1, self.y + 1),
            Position::new(self.x + 1, self.y - 1),
            Position::new(self.x + 1, self.y + 1)
        ]
    }

    fn is_corner_of(&self, diag: Self, region: &HashSet<Self>) -> bool {
        let first_side = Self {
            x: self.x,
            y: diag.y
        };
        let second_side = Self {
            x: diag.x,
            y: self.y
        };
        if region.contains(&first_side) == region.contains(&second_side) {
            let is_inside = region.contains(&first_side);
            !is_inside || !region.contains(&diag)
        } else {
            false
        }
    }

    fn region_from(self, map: &HashMap<Position, char>) -> HashSet<Position>{
        let mut new_region: HashSet<Position> = HashSet::new();
        let mut to_insert = HashSet::from([self]);

        let letter = map.get(&self).unwrap();

        while !to_insert.is_empty() {
            let mut new_to_insert: HashSet<Position> = HashSet::new();
            to_insert.iter().for_each(|pos| { new_region.insert(*pos); });
            for pos in to_insert {
                pos.neighbours().iter()
                    .filter(|neighbour| {
                        if map.contains_key(neighbour) {
                            map.get(neighbour).unwrap() == letter
                        } else {
                            false
                        }
                    })
                    .filter(|neighbour| !new_region.contains(neighbour))
                    .for_each(|neighbour| { new_to_insert.insert(*neighbour); });
            }
            to_insert = new_to_insert;
        }

        new_region
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mut map = HashMap::new();

    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.map_while(Result::ok).enumerate() {
            for (j, letter) in line.chars().enumerate() {
                map.insert(Position::new(i, j), letter);
            }
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let mut regions: Vec<HashSet<Position>> = Vec::new();

    for position in map.keys() {
        if regions.iter().all(|region| !region.contains(position)) {
            regions.push(position.region_from(&map));
        }
    }

    let mut result: usize = 0;

    for region in regions {
        let area = region.len();

        // Translate region to be away from 0
        let translated_region: HashSet<Position> = region.iter().map(|position| Position::new(position.x + 1, position.y + 1)).collect();

        // Find all corners
        let nb_corners = translated_region.clone().into_iter()
            .map(|position| position.diags().into_iter()
                .filter(|diag| position.is_corner_of(*diag, &translated_region))
                .count())
            .sum::<usize>();

        let nb_sides = nb_corners;

        result += nb_sides * area;
    }

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
