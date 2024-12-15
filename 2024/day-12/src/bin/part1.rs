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
        let mut perimeter = 0;

        for position in region.clone() {
            perimeter += 4 - position.neighbours().iter().filter(|neighbour| region.contains(neighbour)).count();
        }

        result += perimeter * area;
    }

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
