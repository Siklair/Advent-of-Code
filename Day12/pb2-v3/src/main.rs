use core::fmt;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use crate::List::{Cons, Nil};

#[derive(Hash)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    fn from_vec(v: Vec<T>) -> Self {
        let mut res = Nil;
        for x in v.into_iter().rev() {
            res = Cons(x, Box::new(res));
        }
        res
    }
}

impl<T> List<T>
    where T: fmt::Display
{
    fn fmt_aux(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cons(a, b) => { write!(f, ", {a}").unwrap(); b.fmt_aux(f) },
            Nil => Ok(())
        }
    }
}

impl<T> PartialEq for List<T>
    where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Nil, Nil) => true,
            (Nil, _) | (_, Nil) => false,
            (Cons(a, b), Cons(c, d)) => a == c && b == d
        }
    }
}

impl<T> Eq for List<T> 
    where T:Eq {}

impl<T> Clone for List<T> 
    where T: Clone
{
    fn clone(&self) -> Self {
        match self {
            Cons(a, b) => Cons(a.clone(), b.clone()),
            Nil => Nil,
        }
    }
}

impl<T> fmt::Display for List<T>
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cons(a, b) => { 
                write!(f, "[{a}").unwrap(); b.fmt_aux(f).unwrap(); write!(f, "]")
            },
            Nil => write!(f, "[]")
        }
    }
}

#[derive(Clone, Hash)]
enum SpringState {
    OPERATIONAL,
    DAMAGED,
    UNKNOWN,
}

impl fmt::Display for SpringState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DAMAGED => write!(f, "damaged"),
            Self::OPERATIONAL => write!(f, "operational"),
            Self::UNKNOWN => write!(f, "unknown"),
        }
    }
}

impl PartialEq for SpringState
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::DAMAGED, Self::DAMAGED) | (Self::OPERATIONAL, Self::OPERATIONAL) | (Self::UNKNOWN, Self::UNKNOWN) => true,
            _ => false
        }
    }
}

impl Eq for SpringState {}

/* struct SpringsLine {
    record: Vec<SpringState>,
    damaged_size: Vec<usize>,
} */

fn nb_combinations(record: List<SpringState>, damaged_size: List<usize>, started_group: bool, memo: &mut HashMap<(List<SpringState>,List<usize>), usize>) -> usize {
    /* println!("#################");
    println!("record: {}", &record);
    println!("damaged_size: {}", &damaged_size);
    println!("started_group: {started_group}"); */
    if memo.contains_key(&(record.clone(), damaged_size.clone())) && !started_group {
        return *memo.get(&(record.clone(), damaged_size.clone())).unwrap();
    }
    let clones = (record.clone(), damaged_size.clone());

    let res = 
        match (record, damaged_size) {
            (Nil, Nil) => 1,
            (Nil, Cons(0, t)) => {
                match *t {
                    Nil => 1,
                    Cons(_, _) => 0,
                }
            },
            (Cons(SpringState::DAMAGED, _), Cons(0, _)) => 0,
            (Cons(_, b), Cons(0, t)) => {
                if started_group { nb_combinations(*b, *t, false, memo) }
                else { 0 }
            },
            (Cons(SpringState::DAMAGED, _), Nil) => 0,
            (Cons(_, b), Nil) => nb_combinations(*b, Nil, false, memo),
            (Nil, Cons(_, _)) => 0,
            (Cons(SpringState::DAMAGED, t), Cons(x, y)) => {
                nb_combinations(*t, Cons(x - 1, y), true, memo)
            },
            (Cons(SpringState::UNKNOWN, t), Cons(x, y)) => {
                if started_group { nb_combinations(*t, Cons(x - 1, y), true, memo) }
                else {
                    let t2 = t.clone();
                    let y2 = y.clone();
                    nb_combinations(*t, Cons(x - 1, y), true, memo) + nb_combinations(*t2, Cons(x, y2), false, memo)
                }
            },
            (Cons(SpringState::OPERATIONAL, t), damaged_size) => {
                if started_group { 0 }
                else { nb_combinations(*t, damaged_size, false, memo) }
            },
        };
    if !started_group {
        memo.insert(clones, res);
    }
    res
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut total = 0;

    let mut memo = HashMap::new();

    if let Ok(lines) = read_lines(file) {
        for (i, line) in lines.enumerate() {
            if let Ok(text) = line {

                let (record_text, damaged_size_text) = text.split_once(' ').unwrap();
                let unfolded_record_text = vec![record_text.clone(), record_text.clone(), record_text.clone(), record_text.clone(), record_text.clone()].join("?");
                let record = unfolded_record_text.chars()
                    .map(
                        |c| match c {
                            '.' => SpringState::OPERATIONAL,
                            '#' => SpringState::DAMAGED,
                            '?' => SpringState::UNKNOWN,
                            _ => unreachable!(),
                        }
                    ).collect::<Vec<_>>();
                let unfolded_damaged_size_text = 
                    vec![damaged_size_text.clone(), damaged_size_text.clone(), damaged_size_text.clone(), damaged_size_text.clone(), damaged_size_text.clone()].join(",");
                let damaged_size: Vec<usize> = unfolded_damaged_size_text.split(',').map(|c| c.parse().unwrap()).collect();

                //let springs_line = SpringsLine{ record, damaged_size };
                total += nb_combinations(List::from_vec(record), List::from_vec(damaged_size), false, &mut memo);

                println!("{}/1000; total = {total}", i+1);

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
