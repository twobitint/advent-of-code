// Solve the problem!
pub fn input() -> &'static str { include_str!("../input/12") }
pub fn solve(input: &str) -> (usize, usize) {
    (p1(input), p2(input))
}

// Test the problem!
#[allow(dead_code)]
pub fn test_input() -> &'static str { include_str!("../input/12_test") }
#[allow(dead_code)]
pub fn test() {
    let input = test_input();
    let (p1, p2) = solve(input);
    assert_eq!(p1, 0);
    assert_eq!(p2, 0);
}

/* Solution Space Below. */

fn p1(input: &str) -> usize {
    0
}

fn p2(input: &str) -> usize {
    0
}