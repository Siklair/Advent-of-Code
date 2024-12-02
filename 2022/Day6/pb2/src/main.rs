use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

struct EnsembleTrie {
    size: usize,
    letters: Vec<char>,
    index: usize,
}

impl EnsembleTrie {
    pub fn new(size: usize) -> EnsembleTrie {
        return EnsembleTrie { 
            size: size,
            letters: Vec::new(),
            index: 0,
        }
    }

    pub fn all_differents(&mut self, c: char) -> bool {
        let index = self.next_index();
        if self.letters.len() == index {
            self.letters.push(' ');
        }
        self.letters[index] = c;
        let mut letters: Vec<char> = Vec::new();
        for letter in &self.letters {
            if !letters.contains(letter) {
                letters.push(*letter);
            }
        }
        return letters.len() == self.size;
    }

    fn next_index(&mut self) -> usize {
        let res = self.index;
        self.index += 1;
        if self.index == self.size {
            self.index = 0;
        }
        return res;
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );
    
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {
                if text != "" {
                    let mut finished = false;
                    let mut k = 0;
                    let mut last_letters = EnsembleTrie::new(14);
                    for letter in text.chars() {
                        k += 1;
                        if last_letters.all_differents(letter) && !finished {
                            println!("Letters are: {:?}\nAt index: {k}", last_letters.letters);
                            finished = true;
                        }
                    }
                }
            }
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
