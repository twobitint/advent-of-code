use std::fs;

pub fn solve() {

    let contents = fs::read_to_string("./src/day1/input.txt").expect("Input failure");
    let cals = contents.split("\n").map(|a| a.trim());

    let mut most = 0;
    let mut sum = 0;

    for cal in cals {

        if cal.len() == 0 {
            if sum > most {
                most = sum;
            }
            sum = 0;
        } else {
            sum += cal.parse::<i32>().expect("Input data is not an integer");
        }

    }

    println!("{most}");
}