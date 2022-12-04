mod d3;

fn main() {

    // let mut left: Vec<u8> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes().iter().map(|a| d3::atop(*a)).collect();
    // println!("{:?}", left);

    let input = include_str!("../input/03");
    let now = std::time::Instant::now();
    let (p1, p2) = d3::solve(input);

    println!("Solution Part1:\t{}", p1);
    println!("Solution Part2:\t{}", p2);
    println!("Elapsed:\t{:.2?}", now.elapsed());
}
