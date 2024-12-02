use std::env;
use std::fs;
use std::cmp;

fn main() {
    let file_path = "input.txt";
    let content = fs::read_to_string(file_path)
        .expect("Can't read the file");
    
    let lines = content.split("\n");

    let mut max_cal = 0;
    let mut cal = 0;
    for line in lines {
        if line == "" {
            max_cal = cmp::max(max_cal, cal);
            cal = 0;
        } else {
            let item_cal : i32 = line.parse().unwrap();
            cal += item_cal;
        }
    }
    print!("Le maximum de calories transporté par un lutin est : {}", max_cal);
}
