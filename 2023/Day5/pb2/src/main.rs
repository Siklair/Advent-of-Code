use core::fmt;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct MyRange { // same as start..end
    pub start: u64,
    pub end: u64,
}

impl MyRange {

    pub fn new(start: u64, end:u64) -> Self {
        Self {
            start, 
            end,
        }
    }

   /*  pub fn contains(&self, x: u64) -> bool {
        self.start <= x && x < self.end
    }
 */
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    pub fn intersect(&self, other: &Self) -> Self {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        Self{start, end,}
    }

    pub fn diff(&self, other: &Self) -> Vec<Self> {
        let r1 = Self{
            start: self.start,
            end: self.end.min(other.start),
        };
        let r2 = Self{
            start: self.start.max(other.end),
            end: self.end,
        };
        let mut res: Vec<MyRange> = Vec::new();
        if !r1.is_empty() {
            res.push(r1);
        }
        if !r2.is_empty() {
            res.push(r2);
        }
        res
    }

}

impl fmt::Display for MyRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut targets: Vec<MyRange> = Vec::new();
    let mut mappings: Vec<Vec<(MyRange, u64)>> = Vec::new(); // (MyRange(start_source, end_source), start_dest)

    // parsing
    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.enumerate() {
            if let Ok(text) = line {

                if i == 0 {
                    let numbers = text.split(": ").nth(1).unwrap().split(' ').map(|s| s.parse::<u64>().unwrap());
                    let mut parity = 0;
                    let mut last: u64 = 0;
                    for number in numbers {
                        if parity % 2 == 0 {
                            last = number;
                        } else {
                            targets.push(MyRange::new(last,last+number));
                        }
                        parity += 1;
                    }
                    //println!("{:#?}", targets);
                } else {
                    if text.is_empty() {
                        continue;
                    } else if text.contains(":") {
                        mappings.push(Vec::new());
                    } else {
                        let nums: Vec<u64> = text.split(' ').map(|s| s.parse().unwrap()).collect();
                        //println!("{} - {}", nums[0], nums[0]+nums[2]);
                        mappings.last_mut().unwrap().push((MyRange::new(nums[1], nums[1]+nums[2]), nums[0]));
                    }
                }
            }
        }
    }

    // computation
    for i in 0..mappings.len() {
        //println!("\n### iteration {} ###", i);
        //'x:for x in targets.iter_mut() {
        let mut j: usize = 0;
        'x: while j < targets.len() {
            let x = targets.get_mut(j).unwrap();
            j += 1;

            //println!("\ntarget: {}", j);
            for (r, y) in mappings.get(i).unwrap().iter() {
                let inter = x.intersect(r);
                if !inter.is_empty() {
                    //println!("inter: {}", inter);
                    let diff = x.diff(r);
                    //print!("{x} : {r}, {} -> ", (*y as i64 - r.start as i64));
                    x.start = y + inter.start - r.start;
                    x.end = y + inter.end - r.start;
                    //print!("{x}");
                    for r_diff in diff {
                        //print!("+ {r_diff}");
                        targets.push(r_diff);
                    }
                    //println!();
                    continue 'x;
                }   
            }
            //println!();
        }
    }

    let total = targets.iter().map(|r| r.start).reduce(|s1, s2| s1.min(s2)).unwrap();

    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
