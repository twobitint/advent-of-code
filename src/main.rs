mod d15;
use d15 as day;

fn main() {

    let input = day::input();
    let now = std::time::Instant::now();
    let (p1, p2) = day::solve(input);
    let elapsed = now.elapsed();

    println!("Solution Part1:\t{}", p1);
    println!("Solution Part2:\t{}", p2);
    println!("Elapsed:\t{:.2?}", elapsed);
}

#[test]
fn test() {
    day::test();
}