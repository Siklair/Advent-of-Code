use crate::Direction::*;
use core::panic;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Graph {
    edges_from: HashMap<Node, Vec<(usize, Node)>>, // v contains (distance, node) tuples
    start: Node,
    goal: Node,
}

type Node = usize;

impl Graph {
    // create the graph from a map representation
    // adjacent paths are connected with an edge of weight 1
    fn from_map(
        map: &HashMap<Position, char>,
        start: Position,
        goal: Position,
        rows: usize,
        cols: usize,
    ) -> Self {
        let mut edges_from = HashMap::new();
        let mut next_node = 0;
        let mut nodes = HashMap::new();
        nodes.insert(start, next_node);
        next_node += 1;
        let mut to_visit = vec![start];
        while !to_visit.is_empty() {
            let pos = to_visit.pop().unwrap();
            let node = *nodes.get(&pos).unwrap();
            let neighs = pos.neighbours(map, rows, cols);
            for neigh in neighs {
                if let Entry::Vacant(e) = nodes.entry(neigh) {
                    e.insert(next_node);
                    next_node += 1;
                    to_visit.push(neigh);
                }
                let neigh_node = nodes.get(&neigh).unwrap();
                let neighs = edges_from.entry(node).or_insert_with(Vec::new);
                neighs.push((1, *neigh_node));
            }
        }
        Self {
            edges_from,
            start: *nodes.get(&start).unwrap(),
            goal: *nodes.get(&goal).unwrap(),
        }
    }

    // verify that all extremities of an edge are in the graph
    fn verify_extremities(&self) -> bool {
        for (_, neighs) in self.edges_from.iter() {
            for (_, neigh) in neighs {
                if !self.edges_from.contains_key(neigh) {
                    println!("{} is not in the graph", neigh);
                    return false;
                }
            }
        }
        true
    }

    // verify that all edges are symmetric
    fn verify_symmetry(&self) -> bool {
        for (node, neighs) in self.edges_from.iter() {
            for (dist, neigh) in neighs {
                if !self
                    .edges_from
                    .get(neigh)
                    .unwrap()
                    .contains(&(*dist, *node))
                {
                    return false;
                }
            }
        }
        true
    }

    // removes all nodes that have arity 2
    // their neighbours are connected to each other
    // with a new edge of weight equal to the sum of the two previous edges
    fn simplify(&mut self) {
        // find all nodes with arity 2
        let to_remove: HashSet<Node> = self
            .edges_from
            .iter()
            .filter(|(_, neighs)| neighs.len() == 2)
            .map(|(node, _)| *node)
            .collect();
        // for each node to remove
        for node in to_remove {
            // find its neighbours
            let neighs = self.edges_from.get(&node).unwrap();
            let (dist1, neigh1) = neighs[0];
            let (dist2, neigh2) = neighs[1];
            let new_dist = dist1 + dist2;
            // add a new edge between the neighbours
            // with a weight equal to the sum of the two previous edges
            // alse remove the old edges from the neighbours
            let neigh1_neighs = self.edges_from.get_mut(&neigh1).unwrap();
            neigh1_neighs.push((new_dist, neigh2));
            neigh1_neighs.retain(|(_, n)| *n != node);
            let neigh2_neighs = self.edges_from.get_mut(&neigh2).unwrap();
            neigh2_neighs.push((new_dist, neigh1));
            neigh2_neighs.retain(|(_, n)| *n != node);
            // remove the node from the graph
            self.edges_from.remove(&node);
        }
    }

    // remove all nodes that have arity 1
    // except the start and the goal
    fn remove_dead_ends(&mut self) {
        // find all nodes with arity 1
        let to_remove: HashSet<Node> = self
            .edges_from
            .iter()
            .filter(|(node, neighs)| {
                **node != self.start && **node != self.goal && neighs.len() == 1
            })
            .map(|(node, _)| *node)
            .collect();
        // for each node to remove
        for node in to_remove {
            // remove the edge from the neighbour
            let neigh = self.edges_from.get_mut(&node).unwrap();
            neigh.pop();
            // remove the node from the graph
            self.edges_from.remove(&node);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn add_direction_nth(&self, direction: Direction, n: usize) -> Self {
        let (dir_x, dir_y) = direction.to_vector();
        Self::new(self.x + dir_x * n as isize, self.y + dir_y * n as isize)
    }

    fn add_direction(&self, direction: Direction) -> Self {
        self.add_direction_nth(direction, 1)
    }

    fn possible_neighbours(&self) -> Vec<Position> {
        vec![
            self.add_direction(North),
            self.add_direction(West),
            self.add_direction(South),
            self.add_direction(East),
        ]
    }

    fn neighbours(&self, map: &HashMap<Position, char>, rows: usize, cols: usize) -> Vec<Position> {
        let mut neighs = self.possible_neighbours();
        neighs.retain(|pos| {
            pos.x >= 0 && pos.x < rows as isize && pos.y >= 0 && pos.y < cols as isize
        });
        neighs.retain(|pos| *map.get(pos).unwrap() != '#');
        neighs
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn to_vector(self) -> (isize, isize) {
        match self {
            North => (-1, 0),
            West => (0, -1),
            South => (1, 0),
            East => (0, 1),
        }
    }
}

fn main() {
    let file = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join(Path::new("input.txt"));

    let mut map = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;

    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.flatten().enumerate() {
            rows = i + 1;
            if rows == 1 {
                cols = line.len()
            }
            for (j, c) in line.chars().enumerate() {
                map.insert(Position::new(i as isize, j as isize), c);
            }
        }
    }

    let starting_pos = Position::new(
        0,
        (0..cols)
            .find(|j| *map.get(&Position::new(0, *j as isize)).unwrap() == '.')
            .unwrap() as isize,
    );
    let goal = Position::new(
        (rows - 1) as isize,
        (0..cols)
            .find(|j| {
                *map.get(&Position::new((rows - 1) as isize, *j as isize))
                    .unwrap()
                    == '.'
            })
            .unwrap() as isize,
    );

    let mut graph = Graph::from_map(&map, starting_pos, goal, rows, cols);
    if !graph.verify_extremities() {
        panic!("graph has invalid extremities");
    }
    if !graph.verify_symmetry() {
        panic!("graph is not symmetric");
    }
    println!("size before simplification: {}", graph.edges_from.len());
    graph.simplify();
    println!("size after simplification: {}", graph.edges_from.len());
    graph.remove_dead_ends();
    println!("size after removing dead ends: {}", graph.edges_from.len());

    println!("start: {}", graph.start);
    println!("goal: {}", graph.goal);
    for (node, neighs) in graph.edges_from.iter() {
        println!("{}: {:?}", node, neighs);
    }

    let res = longest_path_graph(&graph, graph.start, &mut HashSet::from([graph.start]));
    println!("{}", res);
}

// find the longest path in a graph between start and goal
// recursive function
// current_node: node currently visited
// prevs: set of nodes already visited
fn longest_path_graph(graph: &Graph, current_node: Node, prevs: &mut HashSet<Node>) -> usize {
    if current_node == graph.goal {
        return 0;
    }
    let mut neighs = graph.edges_from.get(&current_node).unwrap().clone();
    neighs.retain(|(_, n)| !prevs.contains(n));
    let mut res = 0;
    for (weight, neigh) in neighs {
        prevs.insert(neigh);
        let local_res = weight + longest_path_graph(graph, neigh, prevs);
        if local_res > res {
            res = local_res;
        }
        prevs.remove(&neigh);
    }
    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/*
Input:
- # : walls
- other : path

#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#*/
