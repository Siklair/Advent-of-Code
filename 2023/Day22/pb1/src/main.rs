use std::collections::{HashMap, HashSet, hash_map::Entry};
use std::{env, fmt};
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use nom::{
    IResult,
    bytes::complete::tag,
    multi::separated_list0,
    character::complete::u64,
   };

struct BrickMap {
    map: HashMap<usize, Brick>,
    supports_map: HashMap<usize, HashSet<usize>>, // (i, v) => i support all j in v
}

impl BrickMap {
    fn fall(&mut self, idx: usize) {
        let brick = self.map.get(&idx).unwrap();
        let mut depends_on = Vec::new();
        for j in 0..idx {
            let other = self.map.get(&j).unwrap();
            if other.is_below(brick) {
                depends_on.push((j, other.z_range.end));
            }
        }
        let &(_, highest_below) = depends_on
            .iter()
            .max_by_key(|(_, z)| z)
            .unwrap_or(&(0, 0));
        depends_on.retain(|(_, z)| *z == highest_below);

        // set dependencies
        if depends_on.len() <= 1 {
            for (j, _) in depends_on {
                if let Entry::Vacant(e) = self.supports_map.entry(j) {
                    e.insert(HashSet::from([idx]));
                } else {
                    self.supports_map.get_mut(&j).unwrap().insert(idx);
                }
            }
        }

        // fall
        let mut brick = self.map.get_mut(&idx).unwrap();
        let delta = brick.z_range.start - highest_below;
        brick.z_range.start -= delta;
        brick.z_range.end -= delta;
    }
}

struct Brick {
    x_range: Range<usize>,
    y_range: Range<usize>,
    z_range: Range<usize>,
}

impl Brick {
    fn new(x_range: Range<usize>, y_range: Range<usize>, z_range: Range<usize>) -> Self {
        Self { x_range, y_range, z_range }
    }

    fn is_below(&self, other:&Self) -> bool {
        self.z_range.end < other.z_range.start
        && self.x_range.clone().any(|x| other.x_range.contains(&x))
        && self.y_range.clone().any(|y| other.y_range.contains(&y))
    }
} 

impl fmt::Display for Brick {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "({}, {}, {}) // ({}, {}, {})", 
            self.x_range.start,
            self.y_range.start,
            self.z_range.start,
            self.x_range.end,
            self.y_range.end,
            self.z_range.end)
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut bricks: Vec<Brick> = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let (_, brick) = parse_line(&line).unwrap();
            bricks.push(brick);
        }
    }

    // sort by z ascending
    bricks.sort_by_key(|brick| brick.z_range.start);
    let len = bricks.len();

    let mut brick_map: BrickMap =
        BrickMap {
            map: HashMap::from_iter(
                bricks
                .into_iter()
                .enumerate()
            ),
            supports_map: HashMap::new(),
        };

    for i in 0..len {
        brick_map.fall(i);
    }

    let res = (0..len).filter(|i| !brick_map.supports_map.contains_key(i)).count();

    println!("{res}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// parsing
// 1,0,1~1,2,1
fn parse_xyz(input: &str) -> IResult<&str, (usize, usize, usize)> {
    let (input, pos) = separated_list0(tag(","), u64)(input)?;
    Ok((input, (pos[0] as usize, pos[1] as usize, pos[2] as usize)))
}

fn parse_line(input: &str) -> IResult<&str, Brick> {
    let (input, (x1, y1, z1)) = parse_xyz(input)?;
    let (input, _) = tag("~")(input)?;
    let (input, (x2, y2, z2)) = parse_xyz(input)?;
    Ok((input, Brick::new(x1..x2+1, y1..y2+1, z1..z2+1)))

}
