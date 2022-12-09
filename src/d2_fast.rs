pub fn solve(input: &str) -> (u16, u16) {

    return input.lines().fold((0, 0), |acc, line| {

        let scores = match line {
            "A X" => (4, 3),
            "A Y" => (8, 4),
            "A Z" => (3, 8),
            "B X" => (1, 1),
            "B Y" => (5, 5),
            "B Z" => (9, 9),
            "C X" => (7, 2),
            "C Y" => (2, 6),
            "C Z" => (6, 7),
            _ => (0, 0),
        };

        return (acc.0 + scores.0, acc.1 + scores.1);
    });

}