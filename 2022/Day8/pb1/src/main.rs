use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path};
use std::env;

fn scenic_score(foret: &Vec<Vec<i32>>, i: usize, j: usize) -> u32{
    return score_direction(foret, i, j, 1, 0) 
        * score_direction(foret, i, j, -1, 0)
        * score_direction(foret, i, j, 0, 1)
        * score_direction(foret, i, j, 0, -1)
}

fn score_direction(foret: &Vec<Vec<i32>>, x: usize, y: usize, dx: i32, dy: i32) 
    -> u32
{
    let n = foret.len();
    let mut i = x as i32;
    let mut j = y as i32;
    let mut stop = false; // quand on rencontre un arbre trop grand
    let mut res = 0;
    while !stop && in_range(i+ dx, j + dy, n as i32) {
        i += dx;
        j += dy;
        stop = foret[x][y] <= foret[i as usize][j as usize];
        res += 1;
    }
    return res;
}

fn in_range(i: i32, j: i32, n: i32) -> bool {
    return i < n && i >= 0 && j < n && j >= 0
}

fn main() {
    let file = env::current_dir().unwrap()
        .parent().unwrap()
        .join(
            Path::new("input.txt")
        );

    let mut foret: Vec<Vec<i32>> = Vec::new();
    
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(text) = line {
                if text != "" {            
                    let mut ligne: Vec<i32> = Vec::new();                    
                    for arbre in text.chars() {
                        ligne.push(arbre.to_digit(10).unwrap().try_into().unwrap());
                    }
                    foret.push(ligne);
                }
            }
        }
    }

    let n = foret.len();

    let mut max_scenic_score = 0;

    for i in 0..n {
        for j in 0..n {
            max_scenic_score = core::cmp::max(max_scenic_score, scenic_score(&foret, i, j));
        }
    }

    println!("Max scenic score is: {max_scenic_score}");

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
