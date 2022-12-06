mod d6;

fn main() {

    let input = include_str!("../input/06");
    let now = std::time::Instant::now();
    let (p1, p2) = d6::solve(input);

    println!("Solution Part1:\t{}", p1);
    println!("Solution Part2:\t{}", p2);
    println!("Elapsed:\t{:.2?}", now.elapsed());
}
