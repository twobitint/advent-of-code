pub fn solve(input: &str) -> i32 {

    let lines = input.lines();

    let mut most_1 = 0;
    let mut most_2 = 0;
    let mut most_3 = 0;
    let mut sum = 0;

    for line in lines {
        if line.is_empty() {
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
            sum += line.parse::<i32>().unwrap();
        }
    }

    return most_1 + most_2 + most_3;
}