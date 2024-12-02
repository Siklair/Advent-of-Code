use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

enum Etat {
    PILES,
    SPACE,
    INSTRUCTS,
}

fn read_instruct(s: &str) -> (usize, usize, usize) {
    let mut words = s.split(' ');
    assert!(words.next().unwrap() == "move");
    let nb = words.next().unwrap().parse().unwrap();
    assert!(words.next().unwrap() == "from");
    let source = words.next().unwrap().parse().unwrap();
    assert!(words.next().unwrap() == "to");
    let dest = words.next().unwrap().parse().unwrap();
    return (nb, source, dest);
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );
    
    let mut etat = Etat::PILES;
    let mut nb_piles = 0;

    let mut piles_data: Vec<String> = Vec::new();
    let mut piles: Vec<Vec<char>> = Vec::new();
    
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {
                match etat {
                    Etat::PILES =>
                        if !text.contains('1') {
                            piles_data.push(text);
                        } else {
                            let indices = text.split(' ');
                            for indice in indices {
                                if let Ok(n) = indice.parse::<u32>() {
                                    nb_piles = n;
                                    assert!(n < 10 && n > 0);
                                }
                            }
                            for _ in 0..nb_piles {
                                piles.push(Vec::new());
                            }
                            etat = Etat::SPACE;
                        },
                    Etat::SPACE => 
                        {
                            while !piles_data.is_empty() {
                                let line = piles_data.pop().unwrap();
                                let mut chars = line.chars();
                                chars.next();
                                let mut k = 0;
                                for char in chars {
                                    if k % 4 == 0 && char != ' ' {
                                        piles.get_mut(k / 4).unwrap().push(char);
                                    }
                                    k += 1;
                                }                         
                            }
                            etat = Etat::INSTRUCTS;
                        }
                    Etat::INSTRUCTS => {
                        if text.split(' ').next() != Some("") {
                            let (nb, source, dest) = read_instruct(&text);
                            let mut to_move: Vec<char> = Vec::new();
                            for _ in 0..nb {
                                to_move.push(piles.get_mut(source-1)
                                    .unwrap().pop().unwrap());
                            }
                            for _ in 0..nb {
                                piles.get_mut(dest-1).unwrap()
                                    .push(to_move.pop().unwrap());
                            }
                        }
                    },
                }

            }
        }
    }
    let mut res: Vec<char> = Vec::new();
    for mut pile in piles {
        res.push(pile.pop().unwrap());
    }

    print!("The word is: ");
    for letter in res {
        print!("{letter}");
    }
    print!("\n");

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
