// Solve the problem!
pub fn input() -> &'static str { include_str!("../input/12") }
pub fn solve(input: &str) -> (usize, usize) {
    (0, 0)
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
