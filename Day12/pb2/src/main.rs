use core::fmt;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use crate::List::{Cons, Nil};

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

#[derive(Clone)]
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

/* struct SpringsLine {
    record: Vec<SpringState>,
    damaged_size: Vec<usize>,
} */

// ############
// Idée: record.split(".") => attribuer un certain nombre de numéros de damaged_size à chaque morceau et calculer les combinaisons plus petites en stockant les résultats
// Inspiration: explorer la prog dynamique

fn nb_combinations(record: List<SpringState>, damaged_size: List<usize>, started_group: bool) -> usize {
    /* println!("#################");
    println!("record: {}", &record);
    println!("damaged_size: {}", &damaged_size);
    println!("started_group: {started_group}"); */
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
            if started_group { nb_combinations(*b, *t, false) }
            else { 0 }
        },
        (Cons(SpringState::DAMAGED, _), Nil) => 0,
        (Cons(_, b), Nil) => nb_combinations(*b, Nil, false),
        (Nil, Cons(_, _)) => 0,
        (Cons(SpringState::DAMAGED, t), Cons(x, y)) => {
            nb_combinations(*t, Cons(x - 1, y), true)
        },
        (Cons(SpringState::UNKNOWN, t), Cons(x, y)) => {
            if started_group { nb_combinations(*t, Cons(x - 1, y), true) }
            else {
                let t2 = t.clone();
                let y2 = y.clone();
                nb_combinations(*t, Cons(x - 1, y), true) + nb_combinations(*t2, Cons(x, y2), false)
            }
        },
        (Cons(SpringState::OPERATIONAL, t), damaged_size) => {
            if started_group { 0 }
            else { nb_combinations(*t, damaged_size, false) }
        },
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input_first_lines.txt")
        );

    let mut total = 0;

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
                total += nb_combinations(List::from_vec(record), List::from_vec(damaged_size), false);
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
