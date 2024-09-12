use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

const LEFT: char = 'L';
const RIGHT: char = 'R';

const NODE_SIZE: usize = 3;

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
    let mut starting_points = Vec::new();
    let mut ending_points = Vec::new();

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

                match aaa.chars().nth(NODE_SIZE-1).unwrap() {
                    'A' => starting_points.push(aaa.to_string()),
                    'Z' => ending_points.push(aaa.to_string()),
                    _ => (),
                }

                //println!("{a} -> ({b}, {c})");
                desert_map.add_node(aaa.to_string(), bbb.to_string(), ccc[0..3].to_string());
            }
        }
    }

    // Computation
    let mut nb_steps = 0;
    let mut current_states = starting_points.clone();

    //let nb_instructions = desert_map.instructions.len();

    let mut heads_that_found_z = Vec::new();
    let mut instructions_to_z = Vec::new();

    'a: while !current_states.iter().all(|x| ending_points.contains(x)) {

        for &instruction in desert_map.instructions.iter() {

            nb_steps += 1;

            for (i, current_state) in current_states.iter_mut().enumerate() {
                let (left, right) = desert_map.nodes.get(current_state).unwrap().clone();
                
                if instruction == LEFT {
                    //println!("{current_state} -> {left}");
                    *current_state = left;
                } else if instruction == RIGHT {
                    //println!("{current_state} -> {right}");
                    *current_state = right;
                } else {
                    unreachable!();
                }

                if current_state.chars().nth(2).unwrap() == 'Z' && !heads_that_found_z.contains(&i) {
                    println!("{}", nb_steps);
                    heads_that_found_z.push(i);
                    instructions_to_z.push(nb_steps);
                    if heads_that_found_z.len() == starting_points.len() {
                        break 'a;
                    }
                }
            }
        }
    }

    let mut lcm_val = 1;
    for n in instructions_to_z {
        lcm_val = lcm(lcm_val, n);
    }

    println!("{lcm_val}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
