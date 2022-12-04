use regex::Regex;

pub fn solve(input: &str) -> (u16, u16) {

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    return input.lines().fold((0, 0), |acc, line| {

        // Parse integer range data from input.
        let cap = re.captures(line).unwrap();
        let x0: u16 = (&cap[1]).parse().unwrap();
        let x1: u16 = (&cap[2]).parse().unwrap();
        let y0: u16 = (&cap[3]).parse().unwrap();
        let y1: u16 = (&cap[4]).parse().unwrap();

        let contains = if (x0 >= y0 && x1 <= y1) || (y0 >= x0 && y1 <= x1) { 1 } else { 0 };
        let partial = if (x0 >= y0 && x0 <= y1) || (y0 >= x0 && y0 <= x1) { 1 } else { 0 };

        return (acc.0 + contains, acc.1 + partial);
    });

}