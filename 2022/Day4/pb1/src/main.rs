use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;


fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );
    
    let mut score: u32 = 0;
    
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {
                if text != "" {

                    let mut split = text.split(",").into_iter();
                    let elf1 = split.next().unwrap();
                    let elf2 = split.next().unwrap();
                    let mut split1 = elf1.split("-").into_iter();
                    let mut split2 = elf2.split("-").into_iter();
                    let x1: u32 = split1.next().unwrap().parse().unwrap();
                    let x2: u32 = split1.next().unwrap().parse().unwrap();
                    let y1: u32 = split2.next().unwrap().parse().unwrap();
                    let y2: u32 = split2.next().unwrap().parse().unwrap();
                    if (x1 >= y1 && x2 <= y2) || (x1 <= y1 && x2 >= y2){
                        score += 1;
                    }
                }
            }
        }
    }
    println!("The result is: {score}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
