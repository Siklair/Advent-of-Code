use nom::{
    bytes::complete::tag, character::complete::alpha1, multi::separated_list1, sequence::tuple,
    IResult,
};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NB_REMOVE: usize = 3;

#[derive(Default)]
struct Graph {
    edge_map: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn add_edges_vec(&mut self, from: Node, to: Vec<Node>) {
        self.edge_map
            .entry(from.clone())
            .or_default()
            .extend(to.clone());
        // add reverse edges
        for node in to {
            self.edge_map.entry(node).or_default().push(from.clone());
        }
    }

    fn is_neighbor(&self, node1: &Node, node2: &Node) -> bool {
        if let Some(edges) = self.edge_map.get(node1) {
            edges.contains(node2)
        } else {
            false
        }
    }

    fn get_edges(&self, node: &Node) -> Option<&Vec<Node>> {
        self.edge_map.get(node)
    }

    fn nb_connections_subset(&self, subset: &Vec<Node>) -> usize {
        let mut nb_connections = 0;
        for node in subset {
            if let Some(edges) = self.get_edges(node) {
                // count connections outside subset
                nb_connections += edges.iter().filter(|&x| !subset.contains(x)).count();
            }
        }
        nb_connections
    }

    // if a node is connected to a connected subset
    fn is_connected_to_subset(&self, node: &Node, subset: &Vec<Node>) -> bool {
        if subset.is_empty() {
            return true;
        }
        for node2 in subset {
            if self.is_neighbor(node, node2) {
                return true;
            }
        }
        false
    }

    // finds the only connected subset of size at least one
    // that is connected to the graph through exactly NB_REMOVE connections
    // perform the search while constructing the connected parts
    // if a connected part is found with less than NB_REMOVE connections, stop the search
    fn find_connected_subset(&self) -> Option<Vec<Node>> {
        let mut nodes: Vec<Node> = self.edge_map.keys().cloned().collect();
        let mut subset = Vec::new();
        self.find_connected_subset_rec(&mut nodes, &mut subset)
    }

    // only consider subsets of size <= half the graph size
    // never push a node to the subset if its size would be > half the graph size
    fn find_connected_subset_rec(
        &self,
        nodes: &mut Vec<Node>,
        subset: &mut Vec<Node>,
    ) -> Option<Vec<Node>> {
        if nodes.is_empty() {
            if self.nb_connections_subset(subset) == NB_REMOVE && !subset.is_empty() {
                //assert!(self.is_connected(subset));
                return Some(subset.clone());
            }
            None
        } else {
            let node = nodes.pop().unwrap();
            if subset.len() < self.edge_map.len() / 2 && self.is_connected_to_subset(&node, subset)
            {
                subset.push(node.clone());
                if let Some(result) = self.find_connected_subset_rec(nodes, subset) {
                    return Some(result);
                }
                subset.pop();
            }
            if let Some(result) = self.find_connected_subset_rec(nodes, subset) {
                return Some(result);
            }
            nodes.push(node);
            None
        }
    }
}

type Node = String;

fn main() {
    let file = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join(Path::new("input_test.txt"));

    let mut graph = Graph::default();

    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let (_, (node, edges)) = parse_line(&line).unwrap();
            graph.add_edges_vec(
                node.to_string(),
                edges.iter().map(|x| x.to_string()).collect(),
            );
        }
    }

    let result = graph.find_connected_subset();
    // print subset.size * (graph.size - subset.size)
    if let Some(subset) = result {
        println!("{}", subset.len() * (graph.edge_map.len() - subset.len()));
    } else {
        panic!("no result found");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// parsing
/* jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr */

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, (name, _, children)) =
        tuple((alpha1, tag(": "), separated_list1(tag(" "), alpha1)))(input)?;

    Ok((input, (name, children)))
}
