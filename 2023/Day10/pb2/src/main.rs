use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

struct PipeMap {
    pipe_map: HashMap<Position, char>,
    rows_num: isize,
    col_num: isize,
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
            col_num: 0,
            rows_num: 0,
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

    fn to_three_by_three(&self, c:char) -> Vec<Self> {
        let middle = Position{i: self.i * 3 + 1, j: self.j * 3 + 1};
        let mut res = vec![middle];
        if c == 'S' {
            unreachable!("3x3 repr of S");
        }
        res.append(&mut middle.possible_neighbours(c));
        res
    }

    fn out_of_bound(&self, rows: isize, col: isize) -> bool {
        !(self.i >= 0 && self.i < rows && self.j >= 0 && self.j < col)
    }

    // for a point in the 3x3 grid, tells if it is actually part of the loop
    fn part_of_the_loop(&self, tile_loop: &Vec<Self>) -> bool {
        tile_loop.iter().any(|p| self.i.abs_diff(p.i) <= 1 && self.j.abs_diff(p.j) <= 1)
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

            pipes.rows_num += 1;

            if let Ok(text) = line {

                if pipes.col_num == 0 {
                    pipes.col_num = text.len() as isize;
                }

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

    // Find the interior of the loop
    // Transform each tile into 3x3 tiles
    let rows = pipes.rows_num * 3;
    let col = pipes.col_num * 3;

    // Find the appropriate char for S
    assert!(*pipes.pipe_map.get(pipe_loop.get(0).unwrap()).unwrap() == 'S');
    let next = *pipe_loop.get(1).unwrap();
    let prev = *pipe_loop.last().unwrap();
    let s = *pipe_loop.get(0).unwrap();

    let mut s_symbol: char = ' ';

    let can_be_symbol_next = 
        if next == s.north() {
            vec!['|', 'L', 'J']
        } else if next == s.south() {
            vec!['|', 'F', '7']
        } else if next == s.west() {
            vec!['7', 'J', '-']
        } else if next == s.east() {
            vec!['L', '-', 'F']
        } else {
            unreachable!("S should be adjacent to next");
        };

    let can_be_symbol_prev = 
        if prev == s.north() {
            vec!['|', 'L', 'J']
        } else if prev == s.south() {
            vec!['|', 'F', '7']
        } else if prev == s.west() {
            vec!['7', 'J', '-']
        } else if prev == s.east() {
            vec!['L', '-', 'F']
        } else {
            unreachable!("S should be adjacent to next");
        };

    for symbol in can_be_symbol_next {
        if can_be_symbol_prev.contains(&symbol) {
            s_symbol = symbol;
            break;
        }
    }

    if s_symbol == ' ' {
        unreachable!();
    }

    // Change the symbol for the start of the loop
    pipes.pipe_map.insert(*pipe_loop.get(0).unwrap(), s_symbol);
    
    // Ths loop is not ordered
    let pipe_loop_three_by_three = &pipe_loop.iter().flat_map(|pos| pos.to_three_by_three(*pipes.get(pos).unwrap())).collect();

    // (0, 0) is necessarily outside the loop in the 3x3 grid and the outside area is all connected
    let outside_of_loop = Position{ i : 0, j : 0 }; 
    let outside_filled = flood_fill(outside_of_loop, pipe_loop_three_by_three, rows, col);

    // outside_filled.len() is the number of elements outside the loop in the 3x3 grid
    // and it contains tiles that are part of the original loop (because of the 3x3 split)
    let mut part_of_the_loop_outside = 0;
    for i in outside_filled.iter() {
        if i.part_of_the_loop(&pipe_loop_three_by_three) {
            part_of_the_loop_outside += 1;
        }
    }

    //println!("(9 - 3) * pipe_loop.len() - part_of_the_loop_outside: {} * {} - {} = {}", 9-3, pipe_loop.len(), part_of_the_loop_outside, (9 - 3) * pipe_loop.len() - part_of_the_loop_outside);
    let part_of_the_loop_inside = (9 - 3) * pipe_loop.len() - part_of_the_loop_outside;

    /* println!("rows as usize * col as usize - outside_filled.len() - pipe_loop_three_by_three.len() - part_of_the_loop_inside: {} * {} - {} - {} - {}", 
                rows as usize, col as usize, outside_filled.len(), pipe_loop_three_by_three.len(), part_of_the_loop_inside); */
    let inside_size_3x3 = rows as usize * col as usize - outside_filled.len() - pipe_loop_three_by_three.len() - part_of_the_loop_inside;
    if inside_size_3x3 % 9 != 0 {
        unreachable!();
    }

    let res = inside_size_3x3 / 9;

    println!("{res}");
}

fn flood_fill(starting_point: Position, walls: &Vec<Position>, rows: isize, col: isize) -> Vec<Position> {
    let mut outside_points: Vec<Position> = Vec::new();
    let mut to_explore = vec![starting_point];
    let mut to_be_explored = Vec::new();
    while !to_explore.is_empty() {
        for point in to_explore.iter() {
            for neigh in point.possible_neighbours('S') { // get neighbours in all directions
                if !(neigh.out_of_bound(rows, col) || outside_points.contains(&neigh) || to_explore.contains(&neigh) || to_be_explored.contains(&neigh) || walls.contains(&neigh)) {
                    //println!("let's explore ({}, {})", neigh.i, neigh.j);
                    to_be_explored.push(neigh);
                }
            }
            outside_points.push(*point);
        }
        to_explore = to_be_explored.clone();
        to_be_explored.clear();
    }
    outside_points
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
