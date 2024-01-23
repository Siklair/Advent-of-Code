use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

struct Race {
    time: u64,
    distance_record: u64,
}

impl Race {

    fn push_time(&self) -> (u64, u64) { // min, max

        let delta = self.time.pow(2) - 4 * self.distance_record;

        let t1: f32 = (self.time as f32 - (delta as f32).sqrt()) / 2.;
        let t2: f32 = (self.time as f32 + (delta as f32).sqrt()) / 2.;

        (t1.floor() as u64 + 1, t2.ceil() as u64 - 1)

    }

}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut race = Race{time:0, distance_record:0};

    if let Ok(lines) = read_lines(file) {
        
        let mut line_iter  = lines.into_iter();

        if let Some(Ok(text)) = line_iter.next() {

            let time: u64 = text.split(':').nth(1).unwrap().split_whitespace().collect::<Vec<&str>>().join("").parse().unwrap();

            if let Some(Ok(text2)) = line_iter.next() {

                let dist: u64 = text2.split(':').nth(1).unwrap().split_whitespace().collect::<Vec<&str>>().join("").parse().unwrap();

                race = Race {time: time, distance_record: dist};

            }

        } else {
            panic!();
        }
    }

    let total;

    let (t1, t2) = race.push_time();
    total = t2 + 1 - t1;
    

    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
