mod d5;

fn main() {

    let input = include_str!("../input/05");
    let now = std::time::Instant::now();
    let (p1, p2) = d5::solve(input);

    println!("Solution Part1:\t{}", p1);
    println!("Solution Part2:\t{}", p2);
    println!("Elapsed:\t{:.2?}", now.elapsed());
}
