mod d9;

fn main() {
    let input = include_str!("../input/09_test");
    let now = std::time::Instant::now();
    let (p1, p2) = d9::solve(input);
    let elapsed = now.elapsed();

    println!("Solution Part1:\t{}", p1);
    println!("Solution Part2:\t{}", p2);
    println!("Elapsed:\t{:.2?}", elapsed);
}
