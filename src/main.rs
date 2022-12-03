mod d2fast;

fn main() {

    let input = include_str!("../input/02");
    let now = std::time::Instant::now();
    let (p1, p2) = d2fast::solve(input);

    println!("Solution Part1:\t{}", p1);
    println!("Solution Part2:\t{}", p2);
    println!("Elapsed:\t{:.2?}", now.elapsed());
}
