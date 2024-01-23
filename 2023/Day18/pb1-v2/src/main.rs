use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::ops::Range;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use crate::Direction::*;
use std::collections::hash_map::Entry;

struct Map {
    x_range: Range<isize>,
    y_range: Range<isize>,
    borders: Vec<MyRange>,
}

impl Map {
    fn new() -> Self {
        Self { 
            x_range: 0..0, 
            y_range: 0..0, 
            borders: Vec::new(),
        }
    }

    fn add_range(&mut self, pos: Position, direction: Direction, n: usize) {
        let r: MyRange = MyRange::new(pos, n, direction);
        self.extend_range(&r);
        self.borders.push(r);
    }

    fn extend_range(&mut self, r: &MyRange) {
        if r.source.y < self.y_range.start {
            self.y_range.start = r.source.y;
        } else if r.source.y >= self.y_range.end {
            self.y_range.end = r.source.y+1;
        }

        if r.x_min() < self.x_range.start {
            self.x_range.start = r.x_min();
        }
        if r.x_max() >= self.x_range.end {
            self.x_range.end = r.x_max() + 1;
        }
    }
}

// Vertical Range
#[derive(Clone, Copy)]
struct MyRange {
    source: Position,
    size: usize,
    direction: Direction
}


impl MyRange {

    fn new(source: Position, size: usize, direction: Direction) -> Self {
        Self { 
            source, 
            size, 
            direction,
        }
    }

    fn x_min(&self) -> isize {
        self.source.x + match self.direction {
            North => - (self.size as isize),
            South => 0,
            _ => unreachable!(),
        }
    }

    fn x_max(&self) -> isize {
        self.source.x + match self.direction {
            North => 0,
            South => self.size as isize,
            _ => unreachable!(),
        }
    }

    fn to_range(self) -> Range<isize> {
        let start = 
            match self.direction {
                North | South => self.source.x,
                West | East => self.source.y,
            };
        let end = 
            match self.direction {
                North => self.source.x - self.size as isize,
                South => self.source.x + self.size as isize,
                West => self.source.y - self.size as isize,
                East => self.source.y + self.size as isize,
            }; // not + 1 on purpose
        if start <= end {
            start+1..end+1
        } else {
            end..start
        }
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
        }
    }

    fn add_direction_nth(&self, direction: Direction, n: usize) -> Self {
        let (dir_x, dir_y) = direction.to_vector();
        Self::new(self.x + dir_x * n as isize, self.y + dir_y * n as isize)
    }

    fn origin() -> Self {
        Self::new(0, 0)
    }

    fn inbounds(&self, x_range: &Range<isize>, y_range: &Range<isize>) -> bool {
        x_range.contains(&self.x) && y_range.contains(&self.y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn to_vector(self) -> (isize, isize) {
        match self {
            North => (-1, 0),
            West => (0, -1),
            South => (1, 0),
            East => (0, 1),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'U' => North,
            'L' => West,
            'D' => South,
            'R' => East,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input_test2.txt")
        );

    let mut map = Map::new();
    let mut current_pos = Position::origin();

    let mut horizontal_ranges = Vec::new();

    //parsing
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            let mut split = line.split(' ');
            let direction = Direction::from_char(split.next().unwrap().chars().next().unwrap());
            let num: usize = split.next().unwrap().parse().unwrap();
            //let _color = split.next().unwrap();

            let next_pos = current_pos.add_direction_nth(direction, num);
            // store only vertical ranges
            match direction {
                West | East => {
                    horizontal_ranges.push(
                        MyRange::new(current_pos, num, direction)
                    );
                    current_pos = next_pos;   
                    continue
                },
                _ => (),
            }

            map.add_range(current_pos, direction, num);
            current_pos = next_pos;
        }
    }
    //computation
    // order by x_min ascending
    map.borders.sort_by_key(|r| r.x_min());
    let mut border_iterator = map.borders.iter().peekable();
    let mut current_borders: Vec<MyRange> = Vec::new();
    let mut interior_ranges: Vec<(isize, Range<isize>)> = Vec::new();

    let first_line = map.x_range.start;
    let nb_lines = map.x_range.len();

    for i in map.x_range.clone() {

        // extract from iter all borders with x_min <= i (in fact x_min == i) into current_borders
        while let Some(r) = border_iterator.peek() {
            match r.x_min().cmp(&i) {
                Ordering::Less => unreachable!(),
                Ordering::Equal => current_borders.push(*border_iterator.next().unwrap()),
                Ordering::Greater => break,

            }
        }

        // sort current_borders by y ascending
        current_borders.sort_by_key(|r| r.source.y);

        // use the parity algorithm (and when at the extremities check the directions of the border and the next one)
        // store the ranges that are inside (excluding the borders)
        let mut parity_res = parity_fill_algorithm(i, current_borders.clone());
        interior_ranges.append(&mut parity_res);

        // clear some of the current borders
        // keep only ranges that will be relevant next iteration
        current_borders.retain(|r| r.x_max() > i);

        // show progress
        println!("{}/{}", i - first_line, nb_lines);
    }

    // return the sum of the the lengths of all stored ranges
    let res: usize = interior_ranges.iter().map(|(_, r)| r.len()).sum::<usize>() 
        + map.borders.iter().map(|r| r.size).sum::<usize>() 
        + horizontal_ranges.iter().map(|r| r.size).sum::<usize>();

    print_debug(&map, interior_ranges, horizontal_ranges);

    println!("{res}");
}

fn print_debug(map: &Map, interior_ranges: Vec<(isize, Range<isize>)>, horizontal_ranges: Vec<MyRange>) {

    let mut count_map: HashMap<Position, usize> = HashMap::new();
    let mut range_size_total = 0;

    for r in map.borders.clone() {
        //println!("({}, {}), size: {}, dir: {:?}", r.source.x, r.source.y, r.size, r.direction);
        // iterate over r
        let range = r.to_range();
        //println!("range size: {}", range.len());
        let j = r.source.y;
        let mut size = 0;
        for i in range.clone() {

            size += 1;

            let pos = Position::new(i, j);
            if let Entry::Vacant(e) = count_map.entry(pos) {
                e.insert(1);
            } else {
                *count_map.get_mut(&pos).unwrap() += 1;
            }
        }
        assert!(size == range.len());
        range_size_total += size;
    }

    for i in map.x_range.clone() {
        for j in map.y_range.clone() {
            let pos = Position::new(i, j);
            if let Some(count) = count_map.get(&pos) {
                match count.cmp(&1) {
                    Ordering::Greater => panic!(),
                    Ordering::Equal => print!("#"),
                    Ordering::Less => unreachable!(),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    let total_count: usize = count_map.values().sum();
    println!("total count: {}, range size: {}", total_count, range_size_total);

    for pos in count_map.keys() {
        if !pos.inbounds(&map.x_range, &map.y_range) {
            print!("out of bound ({}, {}), count = {}", pos.x, pos.y, count_map.get(pos).unwrap());
        }
    }
    println!();

    for r in horizontal_ranges {
        let range = r.to_range();
        let i = r.source.x;
        let mut size = 0;
        for j in range.clone() {

            size += 1;

            let pos = Position::new(i, j);
            if let Entry::Vacant(e) = count_map.entry(pos) {
                e.insert(1);
            } else {
                *count_map.get_mut(&pos).unwrap() += 1;
            }
        }
        assert!(size == range.len());
        range_size_total += size;
    }

    for i in map.x_range.clone() {
        for j in map.y_range.clone() {
            let pos = Position::new(i, j);
            if let Some(count) = count_map.get(&pos) {
                match count.cmp(&1) {
                    Ordering::Greater => panic!(), //print!("{count}"),
                    Ordering::Equal => print!("#"),
                    Ordering::Less => unreachable!(),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }

    let total_count: usize = count_map.values().sum();
    println!("total count: {}, range size: {}", total_count, range_size_total);

    for pos in count_map.keys() {
        if !pos.inbounds(&map.x_range, &map.y_range) {
            print!("out of bound ({}, {}), count = {}", pos.x, pos.y, count_map.get(pos).unwrap());
        }
    }

    println!();

    for (i, r) in interior_ranges {
        let mut size = 0;
        for j in r.clone() {

            size += 1;

            let pos = Position::new(i, j);
            if let Entry::Vacant(e) = count_map.entry(pos) {
                e.insert(1);
            } else {
                *count_map.get_mut(&pos).unwrap() += 1;
            }
        }
        assert!(size == r.len());
        range_size_total += size;
    }

    for i in map.x_range.clone() {
        for j in map.y_range.clone() {
            let pos = Position::new(i, j);
            if let Some(count) = count_map.get(&pos) {
                match count.cmp(&1) {
                    Ordering::Greater =>  panic!(), //print!("{count}"),
                    Ordering::Equal => print!("#"),
                    Ordering::Less => unreachable!(),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }

    let total_count: usize = count_map.values().sum();
    println!("total count: {}, range size: {}", total_count, range_size_total);

    for pos in count_map.keys() {
        if !pos.inbounds(&map.x_range, &map.y_range) {
            print!("out of bound ({}, {}), count = {}", pos.x, pos.y, count_map.get(pos).unwrap());
        }
    }

}

// pre: the vertical borders are ordered by y ascending
// returns the horizontal ranges that are inside (excluding borders)
fn parity_fill_algorithm(i: isize, vertical_borders: Vec<MyRange>) -> Vec<(isize, Range<isize>)> {
    let mut bit = 0; // the parity bit
    let mut res = Vec::new();
    let mut current_range = 0..0;

    let mut on_border = false; // if we are on a horizontal border, i.e. an extremity
    let mut last_direction = North; // only relevant when on_border == true

    // part of the below code could be factorized
    for border in vertical_borders {
        if i < border.x_max() && i > border.x_min() { // strictly inside
            bit = (bit + 1)%2; // invert the bit
            if bit == 0 { // we just left an inside part
                current_range.end = border.source.y; // exclude y
                res.push((i,current_range.clone()));
            } else if bit == 1 { // we just entered an inside part
                current_range.start = border.source.y + 1; // exclude y
            } else {
                unreachable!("bit outside of [0, 1]");
            }
        } else if i == border.x_max() || i == border.x_min() { // extremities
            if !on_border { // the horizontal border is starting
                on_border = true;
                last_direction = border.direction;
                // we keep the bit state as it is relevant for when we will exit the horizontal border
                if bit == 1 { // we were inside
                    current_range.end = border.source.y; // exclude y // we stop the range because we excluse the horizontal part
                    res.push((i, current_range.clone()));
                } else if bit == 0 { // we were outside
                    // do nothing
                } else {
                    unreachable!("bit outside of [0, 1]");
                }
            } else { // the horizontal border is ending
                if last_direction == border.direction { // the bit will switch
                    bit = (bit + 1)%2;
                } else { // the bit keeps its value
                    // do nothing
                }
                if bit == 1 { // we are inside
                    current_range.start = border.source.y + 1; // exclude y // we stop the range because we exclude the horizontal part
                    res.push((i, current_range.clone()));
                } else if bit == 0 { // we are outside
                    // do nothing
                } else {
                    unreachable!("bit outside of [0, 1]");
                }
                on_border = false;
            }
        } else {
            unreachable!();
        }
    }
    res
} 

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
