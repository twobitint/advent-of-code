use std::collections::HashMap;

pub fn solve(input: &str) -> (u16, u16) {
    return (marker(input, 4), marker(input, 14));
}

fn marker(input: &str, size: usize) -> u16 {
    let mut map: HashMap<char, bool> = HashMap::new();
    let buf: Vec<char> = input.chars().collect();
    let mut iter = 0;
    
    while map.len() < size {
        if !map.contains_key(&buf[iter]) {
            map.insert(buf[iter], true);
            iter += 1;
        } else {
            map.clear();
        }
    }

    return iter as u16;
}