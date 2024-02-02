use nom::IResult;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn origin() -> Point {
        Point::new(0, 0)
    }

    fn add_direction(&self, direction: Direction, distance: isize) -> Point {
        match direction {
            Direction::Up => Point::new(self.x - distance, self.y),
            Direction::Down => Point::new(self.x + distance, self.y),
            Direction::Left => Point::new(self.x, self.y - distance),
            Direction::Right => Point::new(self.x, self.y + distance),
        }
    }

    // adjacent even if they are diagonal
    fn next_to(&self, other: Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

struct HeadTail {
    head: Point,
    tail: Point,
}

impl HeadTail {
    fn new(head: Point, tail: Point) -> HeadTail {
        HeadTail { head, tail }
    }

    fn init() -> HeadTail {
        HeadTail::new(Point::origin(), Point::origin())
    }

    // Move the head in the given direction one step
    // The tail will follow the head if it is too far away
    fn move_head(&mut self, direction: Direction) {
        // move the head in the given direction
        match direction {
            Direction::Up => {
                self.head.x -= 1;
            }
            Direction::Down => {
                self.head.x += 1;
            }
            Direction::Left => {
                self.head.y -= 1;
            }
            Direction::Right => {
                self.head.y += 1;
            }
        }

        // update the tail
        if !self.head.next_to(self.tail) {
            for d in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if self.head == self.tail.add_direction(d, 2) {
                    self.tail = self.tail.add_direction(d, 1);
                    assert!(self.head.next_to(self.tail));
                    break;
                } else {
                    for perp in d.perpendicular() {
                        if self.head == self.tail.add_direction(d, 2).add_direction(perp, 1) {
                            self.tail = self.tail.add_direction(d, 1).add_direction(perp, 1);
                            assert!(self.head.next_to(self.tail));
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn perpendicular(&self) -> [Direction; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }
}

struct Instruction {
    direction: Direction,
    distance: isize,
}

fn main() {
    let file = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join(Path::new("input.txt"));

    let mut head_tail = HeadTail::init();

    let mut instructions = Vec::new();

    let mut visited = HashSet::new();
    visited.insert(Point::origin());

    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let (_, instruction) = parse_instruction(&line).unwrap();
            instructions.push(instruction);
        }
    }

    for instruction in instructions {
        for _ in 0..instruction.distance {
            head_tail.move_head(instruction.direction);
            visited.insert(head_tail.tail);
        }
    }

    let res = visited.len();

    //_print_visited_as_2d(&visited);

    println!("Result: {}", res);
}

fn _print_visited_as_2d(visited: &HashSet<Point>) {
    let min_x = visited.iter().map(|p| p.x).min().unwrap();
    let max_x = visited.iter().map(|p| p.x).max().unwrap();
    let min_y = visited.iter().map(|p| p.y).min().unwrap();
    let max_y = visited.iter().map(|p| p.y).max().unwrap();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if Point::new(x, y) == Point::origin() {
                print!("s");
            } else if visited.contains(&Point::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/*
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
*/
fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = parse_direction(input)?;
    let (input, _) = nom::character::complete::space1(input)?;
    let (input, distance) = nom::character::complete::digit1(input)?;
    let distance = distance.parse().unwrap();
    Ok((
        input,
        Instruction {
            direction,
            distance,
        },
    ))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    match input.chars().next() {
        Some('R') => Ok((&input[1..], Direction::Right)),
        Some('L') => Ok((&input[1..], Direction::Left)),
        Some('U') => Ok((&input[1..], Direction::Up)),
        Some('D') => Ok((&input[1..], Direction::Down)),
        _ => panic!("Invalid direction"),
    }
}
