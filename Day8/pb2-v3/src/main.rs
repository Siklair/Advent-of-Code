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

    let nb_instructions = desert_map.instructions.len();
    let mut destination_after_instructions: HashMap<String, String> = HashMap::new();

    let mut destinations_encountered: Vec<Vec<String>> = Vec::new(); // destinations encountered by each head after a certain number of repetition of the instruction list
    for s in &starting_points {
        destinations_encountered.push(vec![s.clone()]);
    }

    let mut found_loop: Vec<usize> = Vec::new();

    let mut found_z: Vec<usize> = vec![0, 0, 0, 0, 0, 0];
    let mut num = 0;

    'a: while !current_states.iter().all(|x| ending_points.contains(x)) {

        for (i, current_state) in current_states.iter_mut().enumerate() {
            
            // know where we are going after executing the instruction list from a node
            if destination_after_instructions.contains_key(current_state) {
                *current_state = destination_after_instructions.get(current_state).unwrap().clone();
            } else {

                let old_state = current_state.clone();

                for &instruction in desert_map.instructions.iter() {

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
                }

                destination_after_instructions.insert(old_state, current_state.clone());
            }

            if destinations_encountered.get(i).unwrap().contains(&current_state) {
                if !found_loop.contains(&i) && current_state.chars().nth(2).unwrap() == 'Z'{
                    found_loop.push(i);
                    println!("loop detected for {i}/{} with loop states {:?} at nb_instructions = {}", 
                        starting_points.len(), 
                        destinations_encountered.get(i).unwrap().iter().filter(|s| s.chars().nth(2).unwrap() == 'Z').collect::<Vec<_>>(),
                        nb_steps + nb_instructions
                    );
                    let x = found_z.get_mut(i).unwrap();
                    *x = nb_steps + nb_instructions;
                    num += 1;
                    if num == starting_points.len() {
                        break 'a;
                    }
                }
                if found_loop.contains(&i) && current_state.chars().nth(2).unwrap() == 'Z' {
                    let x = nb_steps + nb_instructions;
                    let x1 = found_z.get_mut(i).unwrap();
                    let loop_size = x - *x1;
                    *x1 = *x1 - loop_size;
                }  
            } else {
                destinations_encountered.get_mut(i).unwrap().push(current_state.clone());
            }

        }

        nb_steps += nb_instructions;

        //println!("{}", nb_steps/1000000);

    }

    let mut lcm_val = 1;
    for n in found_z {
        lcm_val = lcm(lcm_val, n);
    }

    println!("{}", lcm_val);
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
