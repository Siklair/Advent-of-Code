use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

struct Part {
    value: u32,
    len: u32,
    x: u32,
    y: u32, 
}

impl Part {
    pub fn add_digit(&mut self , c: char) {
        self.len += 1;
        self.value = self.value * 10 + c.to_digit(10).unwrap();
    }
}

struct Symbol {
    x: u32,
    y: u32,
}

// infinity norm
fn next_to_part_symb(p: &Part, s: &Symbol) -> bool {
    let x_dist = p.x.abs_diff(s.x);
    if x_dist <= 1 && p.y <= s.y + 1 && p.y+p.len >= s.y {
        return true;
    }
    return false;
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    // Parsing
    let mut parts: Vec<Part> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for (x, line) in lines.enumerate() {
            if let Ok(text) = line {

                let mut y = 0;
                let mut chars = text.chars();

                while let Some(char) = chars.next() {

                    if char.is_ascii_digit() {
                        if let Some(last_part) = parts.last_mut() {
                            if last_part.x == x as u32 && last_part.y + last_part.len == y {
                                last_part.add_digit(char);
                            } else {
                                let part = Part { value: char.to_digit(10).unwrap(), len: 1, x: x as u32, y: y };
                                parts.push(part);
                            }
                        } else {
                            let part = Part { value: char.to_digit(10).unwrap(), len: 1, x: x as u32, y: y };
                            parts.push(part);
                        }
                    } else if char == '.' {
                        // Nothing
                    } else if char == '*' { // Gear
                        symbols.push(Symbol{x: x as u32, y: y});
                    } else { // Other symbols
                        // Nothing
                    }

                    y += 1;

                }

            }
        }
    }

    // Computation
    let mut total = 0;
    'sym: for symbol in symbols.iter() {
        let mut gear_ratio = 1;
        let mut gear_degree = 0;

        for part in parts.iter() {
            if next_to_part_symb(part, symbol) {
                gear_degree += 1;
                if gear_degree > 2 {
                    continue 'sym;
                } else {
                    gear_ratio *= part.value;
                }
            }
        }

        if gear_degree == 2 {
            total += gear_ratio;
        }
    }


    println!("total: {total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
