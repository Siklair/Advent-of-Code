use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use ProducedStones::*;

const INPUT_FILE_NAME: &str = "input.txt";
const ITER_NUMBER: usize = 75;

type Stone = usize;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ProducedStones {
    Single(Stone),
    Pair(Stone, Stone)
}

impl IntoIterator for ProducedStones {
    type Item = Stone;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Single(stone) => vec![stone].into_iter(),
            Pair(s1, s2) => vec![s1, s2].into_iter()
        }
    }
}

fn update_stone(stone: usize) -> ProducedStones {
    if stone == 0 {
        Single(1)
    } else if stone.to_string().len() % 2 == 0 {
        let stone_string = stone.to_string();
        Pair(stone_string[..stone_string.len()/2].parse().unwrap(), stone_string[stone_string.len()/2..].parse().unwrap())
    } else {
        Single(stone * 2024)
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );
    
    let mut stones: HashSet<Stone> = HashSet::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            line.split_whitespace().map(|num| num.parse().unwrap()).for_each(|num| { stones.insert(num); });
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let original_stones = stones.clone();

    // Links between stones
    let mut stone_production: HashMap<Stone, ProducedStones> = HashMap::new();

    // Know, after ITER_NUMBER iterations, how many rocks there are
    for _ in 0..ITER_NUMBER {
        let mut next_stones = HashSet::new();
        // for each current stone, get its next stones
        for stone in &stones {
            let new_stones = update_stone(*stone);
            // store that the stone gives these ones
            if stone_production.contains_key(stone) {
                unreachable!()
            } else {
                stone_production.insert(*stone, new_stones);
            }
            // store the stones for next iteration if not encountered
            new_stones.into_iter()
                .filter(|stone| !stone_production.contains_key(stone) && !stones.contains(stone))
                .for_each(|stone| { next_stones.insert(stone); });
        }
        // Update the stones
        stones = next_stones;
    }

    // For each stone, for each number of iteration, the value of this stone
    let mut stone_values: HashMap<Stone, HashMap<usize, usize>> = HashMap::new();

    for stone in stone_production.keys() {
        stone_values.entry(*stone).or_default().insert(0, 1);
    }

    for i in 1..=ITER_NUMBER {
        for stone in stone_production.keys() {
            let new_value = update_stone(*stone).into_iter()
                .map(|child| {
                    if stone_values.contains_key(&child) {
                        stone_values.get(&child).unwrap().get(&(i-1)).unwrap()
                    } else {
                        &1
                    }
                })
                .sum();
            stone_values.get_mut(stone).unwrap().insert(i, new_value);
        }
    }

    let result: usize = original_stones.iter()
        .map(|stone| stone_values.get(stone).unwrap().get(&ITER_NUMBER).unwrap())
        .sum();

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
