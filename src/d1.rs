
pub fn solve() -> i32 {

    let contents = include_str!("../input/01");

    // This map is me being lazy.
    let lines = contents.lines();

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

    let total = most_1 + most_2 + most_3;
    return total;
}