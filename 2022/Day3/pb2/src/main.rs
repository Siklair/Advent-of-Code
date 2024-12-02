use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

// Get priority of a letter
fn priority(letter: char) -> u32 {
    let mut n = letter as u32;
    //println!("{letter}");
    if n <= ('z' as u32) && n >= ('a' as u32) {
        n = n - ('a' as u32) + 1;
    } else if n <= ('Z' as u32) && n >= ('A' as u32) {
        n = n - ('A' as u32) + 27;
    } else {
        panic!("Error");
    }
    return n;
}

// Get same letter in three strings/arrays
fn same_letter(s1: &str, s2: &str, s3: &str) -> char {
    let mut c = ' ';
    for letter in s1.chars() {
        if s2.contains(letter) && s3.contains(letter) {
            c = letter;
        }
    }
    return c;
}

/* // Split string in two
fn split_string(s: &str) -> (&str, &str) {
    let n = s.len();
    return (&s[0..n/2], &s[n/2..n]);
} */

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );
    
    let mut score: u32 = 0;
    
    if let Ok(lines) = read_lines(file) {
        let mut k = 0;
        let mut s: Vec<String> = vec!["".to_string(), "".to_string(), "".to_string()];
        for line in lines {
            if let Ok(text) = line {
                if text != "" {
                    s[k] = text;
                    k += 1;
                    if k == 3 {
                        k = 0;
                        let c = same_letter(&s[0], &s[1], &s[2]);
                        let priority = priority(c);
                        score += priority;
                    }
                }
            }
        }
    }
    println!("The total priority is: {score}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
