use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

enum moves {
    Rock,
    Paper,
    Scissors
}

fn get_their_move(mv : &str) -> moves {
    get_move(mv, "A", "B", "C")
}

fn get_my_move(mv : &str) -> moves {
    get_move(mv, "X", "Y", "Z")
}

fn get_move(mv : &str, rock : &str, paper : &str, scissors : &str) 
        -> moves {
    if mv == rock {
        return moves::Rock;
    } else if mv == paper {
        return moves::Paper;
    } else if mv == scissors {
        return moves::Scissors;
    } else {
        panic!();
    }
}

fn calculate_points(my_move : moves, their_move : moves) -> i32{
    return move_points(&my_move) + matchup_points(&my_move, &their_move);
}

fn move_points(mv : &moves) -> i32 {
    match mv {
        moves::Rock => 1,
        moves::Paper => 2,
        moves::Scissors => 3,
    }
}

fn matchup_points(my_move : &moves, their_move : &moves) -> i32 {
    let m1 = move_points(my_move);
    let m2 = move_points(their_move);
    if m1 == m2 {
        return 3;
    } else {
        if m1 < m2 {
            if m1 == 1 && m2 == 3 {
                return 6
            } else {
                return 0;
            }
        } else {
            if m1 == 3 && m2 == 1 {
                return 0;
            } else {
                return 6;
            }
        }
    }

}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );
    
    let mut score = 0;
    
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {
                if text != "" {

                    let mut their_move = moves::Rock;
                    let mut my_move = moves::Rock;

                    let mut v = text.split(" ");
                    if let Some(first_move) = v.next() {
                        their_move = get_their_move(first_move);
                    }
                    if let Some(second_move) = v.next() {
                        my_move = get_my_move(second_move);
                    }

                    score += calculate_points(my_move, their_move);
                }
            }
        }
    }
    
    print!("{}", score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}