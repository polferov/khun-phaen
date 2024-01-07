mod board;
mod search;

fn main() {
    let start = board::Board::new(
        1 << 1,
        1 << 9,
        (1 << 0) | (1 << 3) | (1 << 12) | (1 << 15),
        (1 << 13) | (1 << 14) | (1 << 17) | (1 << 18),
    );
    let search = search::search(start);
    println!("Solution length: {}", search.len());
    for b in search.iter() {
        println!();
        println!("{}", b);
    }
}
