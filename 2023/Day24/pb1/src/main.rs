use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use float_cmp::ApproxEq;
use nom::{
    IResult,
    bytes::complete::tag,
    multi::separated_list0,
    character::complete::{i64, space1, space0},
    sequence::{pair, tuple},
   };

const AREA_START: isize = 200000000000000;
const AREA_END: isize = 400000000000000;

struct Line { 
    // X = x + t * dx && Y = y + t * dy
    // t = (X - x) / dx
    // Y = y + (X - x) * dy / dx
    // Y = aX + b
    // => a = dy / dx
    // => b = y - x * dy / dx
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl Line {
    fn new(x: isize, y: isize, dx: isize, dy: isize) -> Self {
        Self { x, y, dx, dy }
    }

    fn future(&self, other: &Position) -> bool {
        self.dx.cmp(&0) == other.x.total_cmp(&(self.x as f64))
    }

    fn parallel_to(&self, other: &Self) -> bool {
        self.get_a().approx_eq(other.get_a(), (0.0, 4))
    }

    fn intersects_with(&self, other: &Self) -> bool {
        if self.parallel_to(other) {
            if self.get_b().approx_eq(other.get_b(), (0.0, 4)) {
                unimplemented!("exactly same lines");
            }
            false
        } else {
            true
        }
    }

    fn intersects_at(&self, other:&Self) -> Option<Position> {
        if !self.intersects_with(other) {
            return None;
        }
        // Y1 = a1 * X1 + b1
        // Y2 = a2 * X2 + b2
        // Y1 = Y2 & X1 = X2 => a1 * X + b1 = a2 * X + b2 => X * (a1 - a2) = b2 - b1 => X = (b2 - b1) / (a1 - a2) 
        // Y = b1 + a1 * (b2 - b1) / (a1 - a2) = (b1 * a1 - b1 * a2 + a1 * b2 - a1 * b1) / (a1 - a2) = (a1 * b2 - a2 * b1) / (a1 - a2)
        let a1 = self.get_a();
        let b1 = self.get_b();
        let a2 = other.get_a();
        let b2 = other.get_b();

        let x_inter = (b2 - b1) / (a1 - a2);
        let y_inter = (a1 * b2 - a2 * b1) / (a1 - a2);
        //assert!(y_inter.approx_eq(x_inter * a1 + b1, (0.0, 4)));

        Some(Position::new(x_inter, y_inter))
    }

    fn get_a(&self) -> f64 {
        self.dy as f64 / self.dx as f64
    }

    fn get_b(&self) -> f64 {
        self.y as f64 - self.x as f64 * self.dy as f64 / self.dx as f64
    }
}

struct Position {
    x: f64,
    y: f64,
}

impl Position {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn within_area(&self, p1: &Position, p2: &Position) -> bool {
        self.x >= p1.x as f64 && self.x <= p2.x as f64 && self.y >= p1.y as f64 && self.y <= p2.y as f64
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut my_lines = Vec::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let (_, a_line) = parse_line(&line).unwrap();
            my_lines.push(a_line);
        }
    }

    let pos_start = Position::new(AREA_START as f64, AREA_START as f64);
    let pos_end = Position::new(AREA_END as f64, AREA_END as f64);

    let mut total = 0;
    for i in 1..my_lines.len() {
        let first_line = my_lines.get(i).unwrap();
        for j in 0..i {
            let second_line = my_lines.get(j).unwrap();
            if let Some(intersect_pos) = first_line.intersects_at(second_line) {
                if intersect_pos.within_area(&pos_start, &pos_end) 
                    && first_line.future(&intersect_pos) 
                    && second_line.future(&intersect_pos)
                {
                    total += 1;
                }
            }
        }
    }

    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// parsing
// 19, 13, 30 @ -2,  1, -2
fn parse_xyz(input: &str) -> IResult<&str, (isize, isize, isize)> {
    let (input, pos) = separated_list0(pair(tag(","), space1), i64)(input)?;
    Ok((input, (pos[0] as isize, pos[1] as isize, pos[2] as isize)))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, (x, y, _)) = parse_xyz(input)?;
    let (input, _) = tuple((space0, tag("@"), space1))(input)?;
    let (input, (dx, dy, _)) = parse_xyz(input)?;
    Ok((input, Line::new(x, y, dx, dy)))
}