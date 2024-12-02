use std::env;
use std::fs;
use std::cmp;

// Update tab to get the sorted array of the top three values from n and tab
fn top_three(n : i32, mut v : Vec<i32>) -> Vec<i32> {
    v.push(n);
    v.sort();
    v.remove(0);
    return v;
}

fn main() {
    let file_path = "input.txt";
    let content = fs::read_to_string(file_path)
        .expect("Can't read the file");
    
    let lines = content.split("\n");

    let mut max_cal = vec!(0, 0, 0);
    let mut cal = 0;
    for line in lines {
        if line == "" {
            max_cal = top_three(cal, max_cal);
            cal = 0;
        } else {
            let item_cal : i32 = line.parse().unwrap();
            cal += item_cal;
        }
    }
    print!("The amount of calories transported by the 3 elves that carry the most is : {}", 
            max_cal.iter().sum::<i32>());


}
