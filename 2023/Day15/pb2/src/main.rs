use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );


    let mut boxes: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {

                if text.contains('\n') { unreachable!() }
                let split = text.split(",");

                for s in split { // only one line

                    let op: char;
                    let label: &str;
                    let value: usize;
                    if s.contains('-') {
                        op = '-';
                        label = &s[..(s.len()-1)];
                        value = 0;
                    } else if s.contains('=') {
                        op = '=';
                        let (lab, val) = s.split_once("=").unwrap();
                        label = lab;
                        value = val.parse().unwrap();
                    } else {
                        unreachable!();
                    }
                    let box_index = hash_algorithm(label);
                    let the_box_opt = boxes.get_mut(&box_index);
                    match the_box_opt {
                        None => {
                            boxes.insert(box_index, Vec::new());
                        },
                        Some(_) => (),
                    }
                    let the_box = boxes.get_mut(&box_index).unwrap();
                    match op {
                        '-' =>  match the_box.iter().position(|(lens, _)| lens == label) {
                                None => (),
                                Some(i) => { the_box.remove(i); } ,
                            },
                        '=' => match the_box.iter().position(|(lens, _)| lens == label) {
                                None => the_box.push((label.to_string(), value)),
                                Some(i) => the_box.get_mut(i).unwrap().1 = value,
                            },
                        _ => unreachable!(),
                    }
                }

            }
        }
    }

    let mut total = 0;

    for i in 0..256 {
        match boxes.get(&i) {
            Some(the_box) => {
                let mut box_total = 0;
                for (j, (_, val)) in the_box.iter().enumerate() {
                    let lens_score = (i+1) * (j+1) * *val;
                    box_total += lens_score;
                }
                total += box_total;
            },
            None => (),
        }
    }

    println!("{total}");
}

fn hash_algorithm(s: &str) -> usize {
    let mut current_value = 0;
    for c in s.chars() {
        current_value += c as usize; // ascii representation of c
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}