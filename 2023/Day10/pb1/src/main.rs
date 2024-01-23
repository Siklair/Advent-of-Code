use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

struct PipeMap {
    pipe_map: HashMap<Position, char>,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    i: isize,
    j: isize,
}

impl PipeMap {

    fn new() -> Self {
        Self {
            pipe_map : HashMap::new(),
        }
    }

    fn get(&self, pos: &Position) -> Option<&char> {
        self.pipe_map.get(pos)
    }


    fn neighbours(&self, pos: &Position) -> Vec<Position> {
        let c = *self.get(pos).unwrap();
        let neigh = pos.possible_neighbours(c);
        let mut res = Vec::new();
        for p in neigh {
            match self.get(&p) {
                Some(c_neigh) => 
                    if p.possible_neighbours(*c_neigh).contains(pos) {
                        res.push(p);
                    },
                None => (),
            }
        }
        res
    }

}

impl Position {

    fn north(&self) -> Position { 
        Position { i: self.i - 1, j: self.j }
    }

    fn south(&self) -> Position {
        Position { i: self.i + 1, j: self.j }
    }

    fn west(&self) -> Position {
        Position { i: self.i, j: self.j-1 }
    }

    fn east(&self) -> Position {
        Position { i: self.i, j: self.j+1 }
    }

    fn possible_neighbours(&self, c: char) -> Vec<Self> {
        match c {
            'S' => vec![self.north(), self.south(), self.west(), self.east()],
            '7' => vec![self.south(), self.west()],
            'F' => vec![self.south(), self.east()],
            'L' => vec![self.north(), self.east()], 
            'J' => vec![self.north(), self.west()],
            '-' => vec![self.west(), self.east()],
            '|' => vec![self.north(), self.south()],
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

    let mut pipes = PipeMap::new();
    let mut starting_point = Position{ i: 0, j: 0 };
    let mut found_s = false;

    // parsing
    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.into_iter().enumerate() {
            if let Ok(text) = line {
                for (j, c) in text.chars().enumerate() {

                    if c != '.' {
                        pipes.pipe_map.insert(Position{i: i as isize, j: j as isize}, c);

                        if c == 'S' {
                            starting_point = Position{i: i as isize, j: j as isize};
                            if found_s {
                                unreachable!();
                            }
                            found_s = true;
                        }

                    }

                }
            }
        }
    }

    // computation
    if !found_s {
        unreachable!();
    }

    // Find the loop
    let mut pipe_loop = Vec::new();
    pipe_loop.push(starting_point);
    'a: loop {
        let neigh = pipes.neighbours(pipe_loop.last().unwrap());
        if neigh.len() != 2 {
            println!("found {} != 2 neighbours", neigh.len());
        }
        
        if pipe_loop.contains(neigh.get(0).unwrap()) {
            if pipe_loop.contains(neigh.get(1).unwrap()) {
                break 'a; // loop is complete
            } else {
                pipe_loop.push(*neigh.get(1).unwrap());
            }
        } else {
            pipe_loop.push(*neigh.get(0).unwrap());
        }
    }

    let res = pipe_loop.len()/2;

    println!("{res}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
