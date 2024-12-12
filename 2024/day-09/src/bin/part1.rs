use std::collections::LinkedList;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Ordering::*;

const INPUT_FILE_NAME: &str = "input.txt";

#[derive(Clone, Copy, Debug)]
struct Page {
    position: Option<usize>,
    length: usize,
    index: usize
}

impl Page {
    fn fill(&self, start_pos: usize, end_pos: usize) -> (Option<Self>, Option<Self>) {
        let size = end_pos - start_pos;
        if size < self.length {
            (Some(Self { position: None, length: size, index: self.index }), Some(Self { position: self.position, length: self.length - size, index: self.index }))
        } else {
            (Some(*self), None)
        }
    }

    fn insert(&self, result: &mut usize, position: &mut usize) {
        *result += ((2 * *position + self.length - 1) * self.length * self.index) / 2;
        *position += self.length;
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mut pages: LinkedList<Page> = LinkedList::new();

    if let Ok(lines) = read_lines(file) {
        let mut is_empty = false;
        let mut position = 0;
        let mut index = 0;
        for line in lines.map_while(Result::ok) {
            for char in line.chars() {
                let size = char.to_digit(10).unwrap() as usize;
                if !is_empty {
                    pages.push_back(Page { position: Some(position), length: size, index });
                    index += 1;
                }
                position += size;
                is_empty = !is_empty;
            }
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let mut result: usize = 0;
    let mut position: usize = 0;

    // TODO cas terminal: il reste next_insert et/ou next_fill à insérer
    // ne pas extraire en avance, et réinsérer les bouts en trop pour fill
    while !pages.is_empty() {
        match position.cmp(&pages.front().unwrap().position.unwrap()) {
            Greater => unreachable!(),
            Equal => {
                pages.front().unwrap().insert(&mut result, &mut position);
                pages.pop_front().unwrap();
            },
            Less => {
                match pages.back().unwrap().fill(position, pages.front().unwrap().position.unwrap()) {
                    (Some(filled_page), Some(remaining_page)) => {
                        filled_page.insert(&mut result, &mut position);
                        pages.pop_back(); 
                        pages.push_back(remaining_page);
                    },
                    (Some(filled_page), None) => {
                        filled_page.insert(&mut result, &mut position);
                        pages.pop_back();
                    },
                    _ => unreachable!()
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
