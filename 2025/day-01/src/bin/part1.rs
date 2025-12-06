use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

struct Dial {
    upper_bound: isize,
    current_value: isize
}

impl Dial {
    fn left(self, distance: isize) -> Self {
        Dial { 
            upper_bound: self.upper_bound, 
            current_value: (self.current_value - distance) % self.upper_bound 
        }
    }

    fn right(self, distance: isize) -> Self {
        Dial { 
            upper_bound: self.upper_bound, 
            current_value: (self.current_value + distance) % self.upper_bound 
        }
    }

    fn turn(self, movement: Movement) -> Self {
        match movement {
            Movement::Left(x) => self.left(x),
            Movement::Right(y) => self.right(y)
        }
    }
}

enum Movement {
    Left(isize),
    Right(isize)
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mut movements = vec![];

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let (m, d) = line.split_at(1);
            if m == "L" {
                movements.push(Movement::Left(d.parse().unwrap()));
            }
            else if m == "R" {
                movements.push(Movement::Right(d.parse().unwrap()));
            } else {
                unreachable!("no movement {} known", m);
            }
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let mut result: isize = 0;
    let mut dial: Dial = Dial { 
        upper_bound: 100, 
        current_value: 50 
    };

    for movement in movements {
        dial = dial.turn(movement);
        if dial.current_value == 0 {
            result += 1;
        }
    }

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
