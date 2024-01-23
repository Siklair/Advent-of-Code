use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::alpha1, multi::separated_list1, sequence::tuple,
    IResult,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, vec};

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

    fn get_edges(&self, node: &Node) -> Option<&Vec<Node>> {
        self.edge_map.get(node)
    }

    // for all NB_REMOVE edges of the graph
    // is the graph still connected if we remove them ?
    // if not return the size of one of the connected subset
    fn connected_subset_size(&mut self) -> Option<usize> {
        let edge_map_clone = self.edge_map.clone();
        let edges = edge_map_clone.iter().flat_map(|(k, v)| {
            v.iter()
                .map(|x| (k.clone(), x.clone()))
                .filter(|(x, y)| x < y)
        });
        let edges_combinations = edges.combinations(NB_REMOVE);
        // for all combinations of edges to remove
        let n = edges_combinations.clone().count();
        let mut i = 0;
        for combination in edges_combinations {
            i += 1;
            println!("{} / {}", i, n);
            // remove edges
            for (node1, node2) in combination.iter() {
                self.edge_map
                    .get_mut(node1)
                    .unwrap()
                    .retain(|x| *x != *node2);
                self.edge_map
                    .get_mut(node2)
                    .unwrap()
                    .retain(|x| *x != *node1);
            }
            // check if graph is still connected
            // to do so, we check the size of the connected subset containing any node
            // if the size is equal to the size of the graph, then the graph is connected
            let mut connected_subset_size = 0;
            let start_node = self.edge_map.keys().next().unwrap();
            let mut visited = Vec::new();
            let mut to_visit = vec![start_node];
            while let Some(node) = to_visit.pop() {
                if !visited.contains(&node) {
                    visited.push(node);
                    connected_subset_size += 1;
                    if let Some(edges) = self.get_edges(node) {
                        for edge in edges {
                            to_visit.push(edge);
                        }
                    }
                }
            }
            if connected_subset_size == self.edge_map.len() {
                // graph is connected
                // add edges back
                for (node1, node2) in combination {
                    self.edge_map.get_mut(&node1).unwrap().push(node2.clone());
                    self.edge_map.get_mut(&node2).unwrap().push(node1.clone());
                }
                continue;
            } else {
                // graph is not connected
                // return the size of one of the connected subset
                return Some(visited.len());
            }
        }
        None
    }
}

type Node = String;

fn main() {
    let file = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join(Path::new("input.txt"));

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

    let result: Option<usize> = graph.connected_subset_size();
    // print subset.size * (graph.size - subset.size)
    if let Some(len) = result {
        println!("{}", len * (graph.edge_map.len() - len));
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
