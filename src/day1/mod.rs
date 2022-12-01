use std::fs;

pub fn solve() {

    let contents = fs::read_to_string("./src/day1/input.txt").expect("Input failure");
    let cals = contents.split("\n").map(|a| a.trim());

    let mut most_1 = 0;
    let mut most_2 = 0;
    let mut most_3 = 0;

    let mut sum = 0;

    for cal in cals {

        if cal.len() == 0 {
            if sum > most_1 {
                most_3 = most_2;
                most_2 = most_1;
                most_1 = sum;
            } else if sum > most_2 {
                most_3 = most_2;
                most_2 = sum;
            } else if sum > most_3 {
                most_3 = sum;
            }
            sum = 0;
        } else {
            sum += cal.parse::<i32>().expect("Input data is not an integer");
        }

    }

    let total = most_1 + most_2 + most_3;
    println!("{total}");
}