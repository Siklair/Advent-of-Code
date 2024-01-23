use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut targets: Vec<u64> = Vec::new();
    let mut mappings: Vec<Vec<(Range<u64>, u64)>> = Vec::new(); // (Range(start_source, end_source), start_dest)

    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.enumerate() {
            if let Ok(text) = line {

                if i == 0 {
                    targets = text.split(": ").nth(1).unwrap().split(' ').map(|s| s.parse().unwrap()).collect();
                    //println!("{:#?}", targets);
                } else {
                    if text.is_empty() {
                        continue;
                    } else if text.contains(":") {
                        mappings.push(Vec::new());
                    } else {
                        let nums: Vec<u64> = text.split(' ').map(|s| s.parse().unwrap()).collect();
                        //println!("{} - {}", nums[0], nums[0]+nums[2]);
                        mappings.last_mut().unwrap().push((nums[1]..nums[1]+nums[2], nums[0]));
                    }
                }
            }
        }
    }

    for i in 0..mappings.len() {
        // find if a range contains x; convert if needed
        'x:for x in targets.iter_mut() {
            for (r, y) in mappings.get(i).unwrap().iter() {
                if r.contains(x) {
                    //print!("{i} range {}-{}: {x} -> ", r.start, r.end);
                    *x = *y + *x-r.start;
                    //println!("{x}");
                    continue 'x;
                }
            }
        }
    }

    let total = targets.iter().min().unwrap();

    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
