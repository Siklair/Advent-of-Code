use core::panic;
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

const NB_TAIL: usize = 9;

struct HeadTail {
    points: [Point; NB_TAIL + 1],
}

impl HeadTail {
    fn init() -> HeadTail {
        HeadTail {
            points: [Point::origin(); NB_TAIL + 1],
        }
    }

    fn last_tail(&self) -> Point {
        self.points[NB_TAIL]
    }

    // Move the head in the given direction one step
    fn move_head(&mut self, direction: Direction) {
        // move the head in the given direction
        match direction {
            Direction::Up => {
                self.points[0].x -= 1;
            }
            Direction::Down => {
                self.points[0].x += 1;
            }
            Direction::Left => {
                self.points[0].y -= 1;
            }
            Direction::Right => {
                self.points[0].y += 1;
            }
        }

        // update the tails
        // TODO: the tails can move diagonally, but not the head
        for i in 1..=NB_TAIL {
            if !self.points[i - 1].next_to(self.points[i]) {
                for d in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    if self.points[i - 1] == self.points[i].add_direction(d, 2) {
                        self.points[i] = self.points[i].add_direction(d, 1);
                        assert!(self.points[i - 1].next_to(self.points[i]));
                        break;
                    } else {
                        for perp in d.perpendicular() {
                            if self.points[i - 1]
                                == self.points[i].add_direction(d, 2).add_direction(perp, 1)
                                || self.points[i - 1]
                                    == self.points[i].add_direction(d, 2).add_direction(perp, 2)
                            {
                                self.points[i] =
                                    self.points[i].add_direction(d, 1).add_direction(perp, 1);
                                assert!(self.points[i - 1].next_to(self.points[i]));
                                break;
                            }
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
            visited.insert(head_tail.last_tail());
        }
    }

    let res = visited.len();

    _print_visited_as_2d(&visited);

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
