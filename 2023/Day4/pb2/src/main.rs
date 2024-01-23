use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

struct Ticket {
    winning: Vec<u32>,
    scratched: Vec<u32>,
    number: u32,
}

impl Ticket {
    pub fn new(w: Vec<u32>, s: Vec<u32>) -> Ticket {
        Ticket {
            winning: w,
            scratched: s,
            number: 1,
        }
    }

    pub fn add(&mut self, n: u32) -> () {
        self.number += n;
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    //parsing
    let mut tickets: Vec<Ticket> = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {
                
                let (_, numbers) = text.split_once(':').unwrap();
                let (win, scratch) = numbers.split_once('|').unwrap();

                let winning = win.split_whitespace().map(|x| x.parse().unwrap()).collect();
                let scratched = scratch.split_whitespace().map(|x| x.parse().unwrap()).collect();

                let ticket = Ticket::new(winning, scratched);
                tickets.push(ticket);
            }
        }
    }

    //computations
    let mut total: u32 = 0;
    for i in 0..tickets.len() {

        let ticket = tickets.get(i).unwrap();

        let mut n = 0;
        
        for w in &ticket.winning {
            if ticket.scratched.contains(w) {
                n += 1;
            }
        }

        total += ticket.number;
        let number = ticket.number;

        for j in 1..n+1 {
            if let Some(next_j) = tickets.get_mut(i+j) {
                next_j.add(number);
            }
        }       
    }

    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
