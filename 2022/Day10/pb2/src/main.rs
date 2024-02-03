use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::Err;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::opt,
    combinator::value, sequence::preceded, IResult,
};
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

struct Cpu {
    x: isize,
    cycle: usize,
    result: isize,
    sprite: HashSet<Position>,
}

impl Cpu {
    fn init() -> Cpu {
        Cpu {
            x: 1,
            cycle: 0,
            result: 0,
            sprite: HashSet::new(),
        }
    }

    fn noop(&mut self) {
        // if | cycle % 40 - x | <= 1, then add pos to sprite
        if (self.cycle as isize % 40 - self.x).abs() <= 1 {
            self.sprite.insert(Position {
                x: self.cycle as isize / 40,
                y: self.cycle as isize % 40,
            });
        }
        self.cycle += 1;
        if self.cycle % 40 == 20 {
            self.result += self.cycle as isize * self.x;
            println!(
                "cycle: {}, x: {}, val: {}, result: {}",
                self.cycle,
                self.x,
                self.cycle as isize * self.x,
                self.result
            )
        }
    }

    // it takes 2 cycles to add a number
    fn addx(&mut self, n: isize) {
        self.noop();
        self.noop();
        self.x += n;
    }

    fn apply(&mut self, cmd: Command) {
        match cmd {
            Command::Noop => self.noop(),
            Command::Addx(n) => self.addx(n),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Noop,
    Addx(isize),
}

fn main() {
    let file = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join(Path::new("input.txt"));

    let mut cpu = Cpu::init();

    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let (_, cmd) = parse_command(&line).unwrap();
            cpu.apply(cmd);
        }
    }

    // print the the sprite
    // with # if present, else with .
    for i in 0..6 {
        for j in 0..40 {
            let pos = Position { x: i, y: j };
            if cpu.sprite.contains(&pos) {
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
noop
addx 3
addx -5
 */
fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, cmd) = alt((
        value(Command::Noop, tag("noop")),
        preceded(
            tag("addx "),
            // number can be negative
            map_res(
                tuple((opt(char('-')), digit1)),
                |(sign, n): (Option<char>, &str)| {
                    let sign = sign.unwrap_or('+');
                    let n = format!("{}{}", sign, n);
                    Ok::<Command, Err<Command>>(Command::Addx(n.parse::<isize>().unwrap()))
                },
            ),
        ),
    ))(input)?;

    Ok((input, cmd))
}
