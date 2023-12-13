use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

struct History {
    lines: Vec<Vec<i64>>,
}

impl History {
    fn new(s: String) -> Self {
        let mut l = Vec::new();
        l.push(s.as_str().split(" ").map(|x| x.parse::<i64>().unwrap()).collect());
        Self {
            lines: l,
        }
    }

    fn calc_variations(&mut self) {
        let mut current_line = self.lines.get(0).unwrap().clone();        

        while !current_line.iter().all(|&x| x == 0) {

            let mut new_line = Vec::new();
            for i in 0..current_line.len()-1 {

                new_line.push(current_line.get(i+1).unwrap() - current_line.get(i).unwrap());

            }
            self.lines.push(new_line.clone());
            current_line = new_line.clone();

        }
    }

    fn _predict(&mut self) -> i64 {
        
        let mut last_number = 0;

        let mut rev_lines = self.lines.iter_mut().rev();
        rev_lines.next(); // the first line contains only 0 and is not useful

        for line in rev_lines {
            last_number = line.last().unwrap() + last_number;
            line.push(last_number);
        }

        last_number
    }

    fn predict_backward(&mut self) -> i64 {
        
        let mut prev_number = 0;

        let mut rev_lines = self.lines.iter_mut().rev();
        rev_lines.next(); // the first line contains only 0 and is not useful

        for line in rev_lines {
            prev_number = line.get(0).unwrap() - prev_number;
            line.insert(0, prev_number);
        }

        prev_number
    }

}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut histories = Vec::new();

    // Parsing
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {

                histories.push(History::new(text));

            }
        }
    }

    // Computation
    let mut total = 0;

    for hist in histories.iter_mut() {
        hist.calc_variations();
        total += hist.predict_backward();
    }

    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
