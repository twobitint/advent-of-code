use scan_fmt::scan_fmt;

pub fn solve(input: &str) -> (u16, u16) {

    return input.lines().fold((0, 0), |acc, line| {

        let (x0, x1, y0, y1) = scan_fmt!(line, "{d}-{d},{d}-{d}", u16, u16, u16, u16).unwrap();

        let contains = if (x0 >= y0 && x1 <= y1) || (y0 >= x0 && y1 <= x1) { 1 } else { 0 };
        let partial = if (x0 >= y0 && x0 <= y1) || (y0 >= x0 && y0 <= x1) { 1 } else { 0 };

        return (acc.0 + contains, acc.1 + partial);
    });

}