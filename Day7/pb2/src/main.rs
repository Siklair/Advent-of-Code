use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Ordering;
use std::collections::HashMap;

struct Hand {
    cards: Vec<char>,
    bid: u32,
    hand_type: i16,
}

impl Hand {

    pub fn new(s: &str, b: u32) -> Self {
        Hand {
            cards : s.chars().collect(),
            bid: b,
            hand_type: -1,
        }
    }

    pub fn hand_type(&self) -> u8 {
        if self.hand_type >= 0 {
            return self.hand_type as u8;
        }

        let mut card_map: HashMap<char, u8> = HashMap::new();
        for c in &self.cards {
            if card_map.contains_key(c) {
                *card_map.get_mut(c).unwrap() += 1;
            } else {
                card_map.insert(*c, 1);
            }
        }

        let counts: Vec<&u8> = card_map.values().collect();
        
        let res;

        if counts.contains(&&5) {
            res = 6;
        } else if counts.contains(&&4) {
            if self.cards.contains(&'J') {
                res = 6;
            } else {
                res = 5;
            }
        } else if counts.contains(&&3) {
            if counts.contains(&&2) {
                if self.cards.contains(&'J') {
                    res = 6;
                } else {
                    res = 4;
                }
            } else {
                if self.cards.contains(&'J') {
                    res = 5;
                } else {
                    res = 3;
                }
            }
        } else if counts.contains(&&2) {
            let mut number_pairs = 0;
            for i in &counts {
                if **i == 2_u8 {
                    number_pairs += 1;
                }
            }
            if self.cards.contains(&'J') { 
                let mut count_j = 0;
                for c in self.cards.iter() {
                    if *c == 'J' {
                        count_j += 1;
                    }
                }
                if number_pairs == 1 {
                    res = 3;
                } else {
                    if count_j == 1 {
                        res = 4;
                    } else {
                        res = 5;
                    }
                }
            } else {
                res = number_pairs;
            }
        } else {
            if self.cards.contains(&'J') {
                res = 1;
            } else {
                res = 0;
            }
        }
        res
    }

    pub fn card_value(c: char) -> Option<u8> {
        for i in 0..ORDERING.len() {
            if ORDERING[i] == c {
                return Some(i as u8);
            } else {
                continue;
            }
        }
        return None;
    }

}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                for i in 0..HAND_SIZE {
                    match Self::card_value(self.cards[i]).unwrap().cmp(&Self::card_value(other.cards[i]).unwrap()) {
                        Ordering::Equal => (),
                        y => {return Some(y);},
                    }
                }
                println!("/!\\ hands of equal values");
                Some(Ordering::Equal) 
            },
            x => Some(x),
        }
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type() == other.hand_type() {
            self.cards.partial_cmp(&other.cards).unwrap() == Ordering::Equal
        } else {
            false
        }
    }
}

const ORDERING: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

const HAND_SIZE: usize = 5;

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );
    
    let mut hands: Vec<Hand> = Vec::new();

    // parsing
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {

                let (cards, bid) = text.split_once(' ').unwrap();
                hands.push(Hand::new(cards, bid.parse().unwrap()));

            }
        }
    }

    // computation
    for hand in hands.iter_mut() {
        hand.hand_type = hand.hand_type() as i16;
    }
    hands.sort();
    let mut total = 0;

    for (i, hand) in hands.iter().enumerate() {
        //println!("rank {} : {:?} with hand type [{}]", i+1, hand.cards, hand.hand_type);
        total += hand.bid * (i+1) as u32;
    }

    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
