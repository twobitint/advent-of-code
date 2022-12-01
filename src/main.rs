mod d1;

fn main() {
    let now = std::time::Instant::now();
    println!("Solution:\t{}", d1::solve());
    println!("Elapsed:\t{:.2?}", now.elapsed());
}
