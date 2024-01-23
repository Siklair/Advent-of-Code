use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

const START: &str = "AAA";
const FINISH: &str = "ZZZ";
const LEFT: char = 'L';
const RIGHT: char = 'R';

struct Map {
    instructions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

impl Map {

    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, c1: String, c2: String, c3: String) {
        self.nodes.insert(c1, (c2, c3));
    }

}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut desert_map = Map::new();

    // Parsing
    if let Ok(lines) = read_lines(file) {
        let mut lines_iter = lines.into_iter();

        // Get the instructions from line 1
        let first_line = lines_iter.next().unwrap().unwrap();
        desert_map.instructions = first_line.chars().collect();

        lines_iter.next();

        // Get the nodes from the rest of the input
        for line in lines_iter {
            if let Ok(text) = line {

                let (aaa, bbbccc) = text.split_once(" = (").unwrap();
                let (bbb, ccc) = bbbccc.split_once(", ").unwrap();

                //println!("{a} -> ({b}, {c})");
                desert_map.add_node(aaa.to_string(), bbb.to_string(), ccc[0..3].to_string());
            }
        }
    }

    // Computation
    let mut nb_steps = 0;
    let mut current_state = START;

    while current_state != FINISH {

        for &instruction in desert_map.instructions.iter() {

            let (left, right) = desert_map.nodes.get(current_state).unwrap();
            
            if instruction == LEFT {
                current_state = left;
            } else if instruction == RIGHT {
                current_state = right;
            } else {
                unreachable!();
            }

            nb_steps += 1;

        }

    }

    println!("{nb_steps}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
