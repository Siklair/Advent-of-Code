use std::env::{self};
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use Tree::*;

const INPUT_FILE_NAME: &str = "input.txt";

/* #[derive(Clone)]
struct Node {
    val: usize,
    zero_node: Option<Box<Node>>,
    one_node: Option<Box<Node>>
} */

/* impl Node {
    fn new() -> Self {
        Self { 
            val: 0,
            zero_node: Option::None,
            one_node: Option::None
        }
    }
} */

#[derive(Clone)]
enum Tree {
    Leaf,
    Node(usize, Box<Tree>, Box<Tree>)
}

impl Tree {
    fn new_leaf() -> Box<Tree> {
        Box::new(Tree::Leaf)
    }

    fn new() -> Self {
        Node(0, Box::new(Leaf), Box::new(Leaf))
    }

    fn add_occurence(&mut self) -> &mut Self {
        match self {
            Tree::Leaf => {
                *self = Tree::Node(1, Tree::new_leaf(), Tree::new_leaf());
            }
            Tree::Node(n, _, _) => *n += 1
        }
        self
    }

    fn get_value(&self) -> Vec<usize> {
        match self {
            Node(1, left, right) => {
                let mut res = Vec::new();
                res.append(&mut left.compute_value(0));
                res.append(&mut right.compute_value(1));
                res
            },
            _ => unreachable!()
        }
    }

    fn compute_value(&self, og_direction: usize) -> Vec<usize> {
        match self {
            Leaf => Vec::new(),
            Node(1, left, right) => {
                let mut l_val = left.compute_value(0);
                let mut r_val = right.compute_value(1);
                let mut res = vec![og_direction];
                match (l_val.is_empty(), r_val.is_empty()) {
                    (false, false) => res,
                    _ => {
                        assert!(l_val.is_empty() || r_val.is_empty());
                        res.append(&mut l_val);
                        res.append(&mut r_val);
                        res
                    }
                }
            },
            _ => unreachable!()
        }
    }
}

fn add_bits(mut val: usize, bits: Vec<usize>) -> usize {
    for bit in bits {
        assert!(bit <= 1);
        val <<= 1;
        val += bit;
    }
    val
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut bits_tree = Tree::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let mut current_node = &mut bits_tree;
            current_node.add_occurence();
            
            for c in line.chars() {
                if let Node(_, zero_node, one_node) = current_node {
                    current_node = match c {
                        '0' => zero_node.add_occurence(),
                        '1' => one_node.add_occurence(),
                        _ => unreachable!()
                    }
                } else {
                    unreachable!()
                }
            } 
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    // get oxygen rating
    let (mut val, mut left, mut right) = match bits_tree.clone() {
        Leaf => unreachable!(),
        Node(n, l, r) => (n, l, r)
    };

    let mut oxygen_rating = 0;
    while val != 1 {
        oxygen_rating <<= 1;
        (val, left, right) = match (*left, *right) {
            (Leaf, Leaf) => unreachable!(),
            (Leaf, Node(n, l, r)) => {
                oxygen_rating += 1;
                (n, l, r)
            },
            (Node(n, l, r), Leaf) => (n, l, r),
            (Node(n1, l1, r1), Node(n2, l2, r2)) => 
            {
                if n2 >= n1 {
                    oxygen_rating += 1;
                    (n2, l2, r2) 
                } else { 
                    (n1, l1, r1) 
                }
            }
        };
    }
    oxygen_rating = add_bits(oxygen_rating, (Node(val, left, right)).get_value());

    let (mut val, mut left, mut right) = match bits_tree {
        Leaf => unreachable!(),
        Node(n, l, r) => (n, l, r)
    };

    let mut co2_rating = 0;
    while val != 1 {
        co2_rating <<= 1;
        (val, left, right) = match (*left, *right) {
            (Node(n1, l1, r1), Node(n2, l2, r2)) => 
            {
                if n1 <= n2 {
                    println!("{n1} <= {n2}");
                    (n1, l1, r1) 
                } else {
                    println!("{n1} > {n2}");
                    co2_rating += 1;
                    (n2, l2, r2) 
                }
            },
            _ => unreachable!()
        };
    } 
    co2_rating = add_bits(co2_rating, (Node(val, left, right)).get_value());

    let result: usize = oxygen_rating * co2_rating;

    println!("ox: {oxygen_rating}, co2: {co2_rating}, Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
