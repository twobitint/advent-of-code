use std::collections::HashSet;

pub fn solve(input: &str) -> (u16, u16) {
    return (marker(input, 4), marker(input, 14));
}

fn marker(input: &str, size: usize) -> u16 {
    let mut hash: HashSet<char> = HashSet::new();
    let buf: Vec<char> = input.chars().collect();
    let mut iter = 0;
    
    while hash.len() < size {
        hash.insert(buf[iter]);
        iter += 1;
    }

    return iter as u16;
}