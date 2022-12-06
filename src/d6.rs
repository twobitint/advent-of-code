pub fn solve(input: &str) -> (u16, u16) {
    return (p1(input), 0);
}

fn p1(input: &str) -> u16 {
    let buf: Vec<char> = input.chars().collect();
    for i in 0..buf.len()-4 {
        if let [a, b, c, d] = &buf[i..i+4] {
            if a != b && a != c && a != d && b != c && b != d && c != d {
                return i as u16 + 4;
            }
        }
    }
    return 0;
}