use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use crate::Direction::*;
use std::collections::BTreeMap;

const MAX_SAME_DIRECTION: usize = 3;

struct DijkstraSolver {
    map: HashMap<Position, usize>,
    rows: usize,
    cols: usize,
    visited: HashMap<State, usize>,
    end_states: Vec<State>,
}

impl DijkstraSolver {
    fn new(rows: usize, cols: usize, map: HashMap<Position, usize>) -> Self {
        Self {
            map,
            rows,
            cols,
            visited: HashMap::from([
                (State::new(0, 0, SOUTH), 0), 
                (State::new(0, 0, EAST), 0),
            ]),
            end_states: vec![State::new(rows-1, cols-1, SOUTH), State::new(rows-1, cols-1, EAST)],
        }
    }

    fn solve(&mut self) -> usize {

        let (rows, cols, map) = (self.rows, self.cols, &self.map);

        let mut to_visit: BTreeMap<usize, Vec<State>> = BTreeMap::new(); // map indexed by the distance from (0, 0) to the state
        for state in self.visited.keys() {
            let neighs = state.neighbours(rows, cols, map);
            for (neighbour, distance) in neighs {
                if to_visit.contains_key(&distance) {
                    to_visit.get_mut(&distance).unwrap().push(neighbour);
                } else {
                    to_visit.insert(distance, vec![neighbour]);
                }
            }
        }

        while !self.finished() {

            // find the closest next state
            let &shortest_distance = to_visit.keys().next().unwrap();
            let closest_states = to_visit.get_mut(&shortest_distance).unwrap();
            let current_state = closest_states.pop().unwrap();
            if closest_states.is_empty() {
                to_visit.remove(&shortest_distance);
            }

            if self.visited.contains_key(&current_state) {
                continue
            }

            // add the neighbours to to_visit (or update their distance)
            let neighs = current_state.neighbours(rows, cols, map);
            for (neighbour, dist) in neighs {
                if self.visited.contains_key(&neighbour) {
                    ()
                } else {
                    let distance = shortest_distance + dist; 
                    if to_visit.contains_key(&distance) {
                        to_visit.get_mut(&distance).unwrap().push(neighbour);
                    } else {
                        to_visit.insert(distance, vec![neighbour]);
                    }
                }
            }

            self.visited.insert(current_state, shortest_distance);
        }        
        // return the distance of (rows-1, cols-1)
        *self.end_states.iter().map(|end_state| self.visited.get(end_state).unwrap()).min().unwrap()
    }

    fn finished(&self) -> bool {
        self.end_states.iter().all(|end_state| self.visited.contains_key(end_state))
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: Position,
    direction: Direction, // direction from which the position is reached
}

impl State {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self { position: Position::new(x, y), direction }
    }

    // returns: list of all neighbours of self and their distance from self
    fn neighbours(&self, rows: usize, cols: usize, map: &HashMap<Position, usize>) -> Vec<(Self, usize)> {
        let (dir1, dir2) = self.direction.perpendicular();
        let mut neigh = Vec::new();
        // dir1 neighbours
        let mut neigh_dir1 = self.position;
        let mut distance_from_self = 0;
        for _ in 0..MAX_SAME_DIRECTION {
            match neigh_dir1.add_direction(dir1, rows, cols) {
                None => break,
                Some(pos1) => {
                    distance_from_self += *map.get(&pos1).unwrap();
                    neigh.push(
                        (Self::new(pos1.x, pos1.y, dir1), distance_from_self)
                    ); 
                    neigh_dir1 = pos1;
                },
            }
        }
        // dir2 neighbours
        let mut neigh_dir2 = self.position;
        distance_from_self = 0;
        for _ in 0..MAX_SAME_DIRECTION {
            match neigh_dir2.add_direction(dir2, rows, cols) {
                None => break,
                Some(pos2) => {
                    distance_from_self += *map.get(&pos2).unwrap();
                    neigh.push(
                        (Self::new(pos2.x, pos2.y, dir2), distance_from_self)
                    ); 
                    neigh_dir2 = pos2;
                },
            }
        }
        neigh
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }

    fn add_direction(&self, direction: Direction, rows: usize, cols: usize) -> Option<Self> {
        let (x, y) = (self.x as isize, self.y as isize);
        let (dir_x, dir_y) = direction.to_vector();
        let (new_x, new_y) = (x + dir_x, y + dir_y);
        if out_of_bounds(new_x, new_y, rows, cols) {
            None
        } else {
            Some(Self::new(new_x as usize, new_y as usize))
        }
    }

    
}

fn out_of_bounds(x: isize, y: isize, rows: usize, cols: usize) -> bool {
    return !(x >= 0 && x < rows as isize && y >= 0 && y < cols as isize);
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

    fn perpendicular(&self) -> (Self, Self) {
        match self {
            NORTH | SOUTH => (WEST, EAST),
            WEST | EAST => (NORTH, SOUTH),
        }
    }
}

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
        for (i, line) in lines.enumerate() {
            if let Ok(text) = line {
                if rows == 0 { cols = text.len(); }
                rows = i + 1;

                for (j, c) in text.chars().enumerate() {
                    map.insert(Position::new(i, j), c.to_digit(10).unwrap() as usize);
                }
            }
        }
    }

    let mut solver = DijkstraSolver::new(rows, cols, map);

    let res = solver.solve();

    println!("{res}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
