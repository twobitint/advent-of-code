use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

pub fn solve(input: &str) -> (String, String) {

    let (stacks, procedure) = input.split_once("\n\n").unwrap();

    let mut s1 = stacks_from_input(stacks);
    let mut s2 = s1.clone();

    s1.reorder(procedure, false);
    s2.reorder(procedure, true);

    return (solution_from_stacks(s1), solution_from_stacks(s2));
}

trait Reorder {
    fn reorder(&mut self, input: &str, chunk: bool);
}
impl Reorder for HashMap<usize, Vec<char>> {
    fn reorder(&mut self, input: &str, chunk: bool) {
        for line in input.lines() {
            let (count, a, b) = scan_fmt!(line, "move {d} from {d} to {d}", u16, usize, usize).unwrap();
            if chunk {
                let len = self.get(&a).unwrap().len();
                let items: Vec<char> = self.get_mut(&a).unwrap().drain((len-count as usize)..).collect();
                for item in items {
                    self.get_mut(&b).unwrap().push(item);
                }
            } else {
                for _ in 0..count {
                    let item = self.get_mut(&a).unwrap().pop().clone().unwrap();
                    self.get_mut(&b).unwrap().push(item);
                }
            }
        }
    }
}

fn stacks_from_input(input: &str) -> HashMap<usize, Vec<char>> {

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();

    for line in input.lines().rev() {
        let s = String::from(line);

        let mut iter = 0;
        while iter < s.len() {

            let slice = &s[iter..iter+3];

            if let Ok(item) = scan_fmt!(slice, "[{}]", char) {
                let bin = iter / 4 + 1;
                let stack = stacks.entry(bin).or_default();
                stack.push(item);
            }

            iter += 4;
        }
    }

    return stacks;
}

// Generate a string representing the top item of each stack.
fn solution_from_stacks(stacks: HashMap<usize, Vec<char>>) -> String {
    return stacks
        .iter()
        .sorted_by_key(|i| { i.0 })
        .fold(String::from(""), |mut acc, s| {
            acc.push(*s.1.last().unwrap());
            return acc;
    });
}