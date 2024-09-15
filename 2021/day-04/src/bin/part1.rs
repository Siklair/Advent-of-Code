use std::cmp::max;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

#[derive(Clone)]
struct Board {
    content: HashMap<usize, (usize, usize)>,
    rows: Vec<usize>, // rows[i] is the number of unchecked numbers in row i
    cols: Vec<usize>
} 

impl Board {
    fn new() -> Self {
        Self {
            content: HashMap::new(),
            rows: Vec::new(),
            cols: Vec::new(),
        }
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );
    
    let mut drawn_numbers: Vec<usize> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut current_board: Board = Board::new();

    let mut nb_columns: usize = 0;
    let mut nb_rows: usize = 0;

    // parsing
    if let Ok(lines) = read_lines(file) {
        let mut line_number = 1;
        let mut row = 0;
        for line in lines.map_while(Result::ok) {
            if line_number == 1 {
                drawn_numbers = line.split(',').map(|num| num.parse().unwrap()).collect();
            } else if line.is_empty() {
                if line_number != 2 {
                    boards.push(current_board.clone());
                }
                current_board = Board::new();
                row = 0;
            } else {
                for (column, val) in line.split_whitespace().map(|v| v.parse().unwrap()).enumerate() {
                    current_board.content.insert(val, (row, column));
                    nb_rows = max(nb_rows, row+1);
                    nb_columns = max(nb_columns, column+1);
                }
                row += 1;
            }
            line_number += 1;
        }
        boards.push(current_board);
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    for board in &mut boards {
        board.rows = vec![nb_columns;nb_rows];
        board.cols = vec![nb_rows;nb_columns];
    }

    // computation
    let mut result: usize = 0;

    'a: for num in drawn_numbers {
        for board in &mut boards {
            if board.content.contains_key(&num) {
                let (row, col) = *board.content.get(&num).unwrap();
                board.content.remove(&num);
                *board.rows.get_mut(row).unwrap() -= 1;
                *board.cols.get_mut(col).unwrap() -= 1;
                if *board.rows.get(row).unwrap() == 0 || *board.cols.get(col).unwrap() == 0 {
                    result = board.content.keys().sum::<usize>() * num;
                    break 'a;
                }
            }
        }
    }

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
