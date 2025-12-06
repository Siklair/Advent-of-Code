use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input_test.txt";

struct Range {
    begin: usize,
    end: usize
}

fn get_nb_digit(number: usize) -> usize {
    number.to_string().len()
}

// Used to get the number of invalid IDs among numbers with 2*k digits
fn get_invalid_id_count_with_pair_digit_number(k: usize) -> usize {
    9 * get_power_of_10(k-1)
}

fn get_power_of_10(k: usize) -> usize{
    10_usize.pow(k as u32)
}

fn is_pair(k: usize) -> bool {
    k % 2 == 0
}

fn get_invalid_id_count_with_same_digit_number_below_value(k: usize, first_part: usize, last_part:usize, strictness: bool) -> usize {
    assert_eq!(k, get_nb_digit(first_part));
    //assert_eq!(k, get_nb_digit(last_part));
    let mut result= first_part - get_power_of_10(k-1) + 1;
    if last_part > first_part || (!strictness && last_part == first_part) {
        result += 1;
    }
    result
}

fn get_invalid_id_count_with_same_digit_number_above_value(k: usize, first_part: usize, last_part:usize, strictness: bool) -> usize {
    get_invalid_id_count_with_pair_digit_number(k) - get_invalid_id_count_with_same_digit_number_below_value(k, first_part, last_part, !strictness)
}

fn split_number_in_half(number: usize) -> (usize, usize, usize) {
    let nb_digit = get_nb_digit(number);
    assert!(is_pair(nb_digit));
    let number_str = number.to_string();
    let (first_part, last_part) = number_str.split_at(nb_digit/2);
    (nb_digit/2, first_part.parse().unwrap(), last_part.parse().unwrap())
}

fn get_invalid_id_count_between(x: usize, y: usize) -> usize {
    let (n, m) = (get_nb_digit(x), get_nb_digit(y));
    let mut result = 0;
    for k in n+1..m {
        if is_pair(k) {
            result += get_invalid_id_count_with_pair_digit_number(k/2);
        }
    }
    if is_pair(n) {
        let (k, first_part, last_part) = split_number_in_half(x);
        result += get_invalid_id_count_with_same_digit_number_above_value(k, first_part, last_part, false);
    }
    if is_pair(m) {
        let (k, first_part, last_part) = split_number_in_half(y);
        result += get_invalid_id_count_with_same_digit_number_below_value(k, first_part, last_part, false);
        
        if n == m {
            result -= get_invalid_id_count_with_pair_digit_number(k);
        }
    }

    result
}

fn main() {
    let file = env::current_dir().unwrap()
        .join(
            Path::new(INPUT_FILE_NAME)
        );

    let mut ranges = vec![]; 

    if let Ok(lines) = read_lines(file) {
        for line in lines.map_while(Result::ok) {
            let intervals = line.split(',');
            for interval in intervals {
                let (begin, end) = interval.split_once('-').unwrap();
                ranges.push(Range {
                    begin: begin.parse().unwrap(),
                    end: end.parse().unwrap()
                });
            }
        }
    } else {
        println!("File not found: {}", INPUT_FILE_NAME);
    }

    let mut result: usize = 0;

    for range in ranges {
        result += get_invalid_id_count_between(range.begin, range.end);
    }

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
