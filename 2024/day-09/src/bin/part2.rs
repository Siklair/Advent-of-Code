use std::collections::LinkedList;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

#[derive(Clone, Copy, Debug)]
struct Page {
    position: Option<usize>,
    length: usize,
    index: usize
}

fn insert(pages: &mut LinkedList<Page>) -> usize {
    let mut position = 0;
    let mut index = None;
    let mut to_insert = *pages.back().unwrap();
    for (i, page) in pages.iter().enumerate() {
        if page.position.unwrap() - position >= to_insert.length {
            index = Some(i);
            break;
        }
        position = page.position.unwrap() + page.length
    }
    let result: usize;
    if let Some(i) = index {
        to_insert.position = Some(position);
        let mut tail = pages.split_off(i);
        pages.push_back(to_insert);
        pages.append(&mut tail);
        result = ((2 * position + to_insert.length - 1) * to_insert.length * to_insert.index) / 2
    } else {
        result = ((2 * to_insert.position.unwrap() + to_insert.length - 1) * to_insert.length * to_insert.index) / 2
    }
    pages.pop_back();
    result
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mut pages: LinkedList<Page> = LinkedList::new();
    let mut index = 0;

    if let Ok(lines) = read_lines(file) {
        let mut is_empty = false;
        let mut position = 0;
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
    index -= 1;

    while !pages.is_empty() {
        while pages.back().unwrap().index > index {
            pages.pop_back();
        }   
        assert_eq!(index, pages.back().unwrap().index);
        result += insert(&mut pages);
        if index == 0 {
            break;
        }
        index -= 1;
    }

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
