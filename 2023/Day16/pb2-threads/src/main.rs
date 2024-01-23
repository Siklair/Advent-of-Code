use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use std::thread;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

impl Direction {
    fn to_vector(&self) -> (isize, isize) {
        match self {
            Self::NORTH => (-1, 0),
            Self::WEST => (0, -1),
            Self::SOUTH => (1, 0),
            Self::EAST => (0, 1),
        }
    }
}

#[derive(Clone, Copy)]
struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
    rows: usize,
    cols: usize,
}

impl PartialEq for Beam {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.direction == other.direction
    }
}

impl Beam {

    fn new_beams(&self, tile: char) -> Vec<Option<Self>> {
        match tile {
            '.' => vec![self.take_direction(self.direction)],
            '-' => match self.direction {
                Direction::WEST | Direction::EAST => vec![self.take_direction(self.direction)],
                Direction::NORTH | Direction::SOUTH => vec![self.take_direction(Direction::WEST), self.take_direction(Direction::EAST)],
            },
            '|' => match self.direction {
                Direction::NORTH | Direction::SOUTH => vec![self.take_direction(self.direction)],
                Direction::WEST | Direction::EAST => vec![self.take_direction(Direction::NORTH), self.take_direction(Direction::SOUTH)],
            },
            '/' => match self.direction {
                Direction::NORTH => vec![self.take_direction(Direction::EAST)],
                Direction::WEST => vec![self.take_direction(Direction::SOUTH)],
                Direction::SOUTH => vec![self.take_direction(Direction::WEST)],
                Direction::EAST => vec![self.take_direction(Direction::NORTH)],
            },
            '\\' => match self.direction {
                Direction::NORTH => vec![self.take_direction(Direction::WEST)],
                Direction::WEST => vec![self.take_direction(Direction::NORTH)],
                Direction::SOUTH => vec![self.take_direction(Direction::EAST)],
                Direction::EAST => vec![self.take_direction(Direction::SOUTH)],
            },
            _ => unreachable!(),
        }
    }

    fn take_direction(&self, dir: Direction) -> Option<Self> {
        let dir_vec = dir.to_vector();
        let next_x = self.x as isize + dir_vec.0;
        let next_y = self.y as isize + dir_vec.1;
        
        if out_of_bounds(self.rows, self.cols, next_x, next_y) {
            None
        } else {
            Some(Self {
                x: next_x as usize,
                y: next_y as usize,
                direction: dir,
                rows: self.rows,
                cols: self.cols,
            })
        }
    }

}

fn out_of_bounds(rows: usize, cols: usize, x: isize, y: isize) -> bool {
    return !(x >= 0 && x < rows as isize && y >= 0 && y < cols as isize);
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut rows = 0;
    let mut cols = 0;

    let mut map: HashMap<(usize, usize), char> = HashMap::new();

    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.enumerate() {
            if let Ok(text) = line {
                rows = i+1;
                if rows == 1 {
                    cols = text.len();
                }

                for (j, c) in text.chars().enumerate() {
                    map.insert((i, j), c);
                }
            }
        }
    }

    let mut init_beams = Vec::new();

    // north wall
    for j in 0..cols {
        let init_beam = Beam { x:0, y:j, direction: Direction::SOUTH, rows, cols };
        init_beams.push(init_beam);
    }
    // west wall
    for i in 0..rows {
        let init_beam = Beam { x:i, y:0, direction: Direction::EAST, rows, cols };
        init_beams.push(init_beam);
    }

    // south wall
    for j in 0..cols {
        let init_beam = Beam { x:rows-1, y:j, direction: Direction::NORTH, rows, cols };
        init_beams.push(init_beam);
    }
    // east wall
    for i in 0..rows {
        let init_beam = Beam { x:i, y:cols-1, direction: Direction::WEST, rows, cols };
        init_beams.push(init_beam);
    }

    let start = SystemTime::now();
    let size = init_beams.len();
    let init_iter = Arc::new(Mutex::new(init_beams.into_iter().enumerate()));
    let res = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();

    for _ in 0..4 {
        let init_iter = Arc::clone(&init_iter);
        let map = map.clone();
        let res = Arc::clone(&res);
        let handle = thread::spawn(move || {
            let mut local_res = 0;
            loop {
                let (num, init_beam) = match init_iter.lock().unwrap().next() {
                    Some((i, b)) => (i, b),
                    None => break,
                };

                local_res = local_res.max(calc_energized(init_beam, &map, rows, cols));
                println!("{}/{}, time: {}s", num, size, SystemTime::now().duration_since(start).unwrap().as_secs());
            }
            let mut global_res = res.lock().unwrap();
            *global_res = global_res.max(local_res);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", res.lock().unwrap());
}

fn calc_energized(init_beam: Beam, map: &HashMap<(usize, usize), char>, _rows: usize, _cols: usize) -> usize {

    let mut beams: Vec<Beam> = Vec::new();
    let mut current_beams: Vec<Beam> = Vec::new();

    current_beams.push(init_beam);

    
    let mut energized_map: HashSet<(usize, usize)> = HashSet::new();

    loop {
        if current_beams.is_empty() {
            break
        }

        let current_beam = current_beams.pop().unwrap();

        energized_map.insert((current_beam.x, current_beam.y));
        //energized_print(&energized_map, rows, cols);
        
        beams.push(current_beam);
        let (i, j) = (current_beam.x, current_beam.y);
        let mut next_beams= 
            current_beam.new_beams(*map.get(&(i, j)).unwrap())
            .iter()
            .filter(|b| {
                match b {
                    None => false,
                    _ => true,
                }
            })
            .map(|b| b.unwrap())
            .filter(|b| !(current_beams.contains(b) || beams.contains(b)))
            .collect();
        current_beams.append(&mut next_beams);
    }

    //energized_print(&energized_map, rows, cols);
    energized_map.len()
}

fn _energized_print(energized: &HashSet<(usize, usize)>, rows: usize, cols: usize) {
    for i in 0..rows {
        for j in 0..cols {
            if energized.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
