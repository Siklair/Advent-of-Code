use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILE_NAME: &str = "input.txt";

struct Range {
    begin: usize,
    end: usize
}

fn is_pair(k: usize) -> bool {
    k % 2 == 0
}

fn get_nb_digit(number: usize) -> usize {
    number.to_string().len()
}

fn get_power_of_10(k: usize) -> usize{
    10_usize.pow(k as u32)
}

fn split_number_in_half(number: usize) -> (usize, usize, usize) {
    let nb_digit = get_nb_digit(number);
    assert!(is_pair(nb_digit));
    let number_str = number.to_string();
    let (first_part, last_part) = number_str.split_at(nb_digit/2);
    (nb_digit/2, first_part.parse().unwrap(), last_part.parse().unwrap())
}

fn sum_number_between(n: usize, m:usize) -> usize {
    if n <= m {
        (m-n+1)*(m+n)/2
    } else {
        0
    }
}

fn sum_invalid_ids_between(n: usize, m:usize) -> usize {
    let start: usize;
    if is_pair(get_nb_digit(n)) {
        let (_, first, last) = split_number_in_half(n);
        if last <= first {
            start = first;
        } else {
            start = first + 1;
        }
    } else {
        start = get_power_of_10(get_nb_digit(n) / 2);
    }

    let end: usize;
    if is_pair(get_nb_digit(m)) {
        let (_, first, last) = split_number_in_half(m);
        if last >= first {
            end = first;
        } else {
            end = first - 1;
        }
    } else {
        end = get_power_of_10(get_nb_digit(n) / 2) - 1;
    }

    let mut result = 0;

    if end >= start {
        let min_digit = get_nb_digit(start);
        let max_digit = get_nb_digit(end);


        for k in min_digit+1..max_digit {
            result += sum_number_between(get_power_of_10(k-1), get_power_of_10(k)-1) * (1 + get_power_of_10(k));
        }

        result += sum_number_between(start, get_power_of_10(min_digit)-1) * (1 + get_power_of_10(min_digit));
        result += sum_number_between(get_power_of_10(max_digit-1), end) * (1 + get_power_of_10(max_digit));
        if min_digit == max_digit {
            result -= sum_number_between(get_power_of_10(min_digit-1), get_power_of_10(min_digit)-1) * (1 + get_power_of_10(min_digit))
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
        result += sum_invalid_ids_between(range.begin, range.end);
    }

    println!("Result: {}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
