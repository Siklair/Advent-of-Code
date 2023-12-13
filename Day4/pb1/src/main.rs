use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

struct Ticket {
    winning: Vec<u32>,
    scratched: Vec<u32>,
}

impl Ticket {
    pub fn new(w: Vec<u32>, s: Vec<u32>) -> Ticket {
        Ticket {
            winning: w,
            scratched: s,
        }
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
    for ticket in tickets {
        let mut n = 0;
        
        for w in ticket.winning {
            if ticket.scratched.contains(&w) {
                n += 1;
            }
        }

        if n >= 1 {
            total += 2_u32.pow(n - 1);
        }        
    }

    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
