const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

const DRAW: u8 = 2;
const WIN: u8 = 3;

pub fn solve(input: &str) -> (u16, u16) {

    return input.lines().fold((0, 0), |acc, line| {

        let (a, b) = decode(line);

        // Using part 1 strategy.
        let mut score1 = b;
        if b == a {
            score1 += (DRAW - 1) * 3;
        } else if (b == ROCK && a == SCISSORS)
               || (b == PAPER && a == ROCK)
               || (b == SCISSORS && a == PAPER) {
            score1 += (WIN - 1) * 3;
        }

        // Using part 2 strategy.
        let mut score2 = (b - 1) * 3;
        if a == ROCK {
            score2 += if b == WIN { PAPER } else { if b == DRAW { ROCK } else { SCISSORS } };
        } else if a == PAPER {
            score2 += if b == WIN { SCISSORS } else { if b == DRAW { PAPER } else { ROCK } };
        } else {
            score2 += if b == WIN { ROCK } else { if b == DRAW { SCISSORS } else { PAPER } };
        }

        return (acc.0 + score1 as u16, acc.1 + score2 as u16);
    });

}

// Decode elf cypher into RPS values.
fn decode(line: &str) -> (u8, u8) {
    let encoded = line.as_bytes();
    let a = encoded[0] - 64;
    let b = encoded[2] - 87;
    return (a, b);
}