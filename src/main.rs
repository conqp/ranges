use ranges::Ranges;
use std::io::stdin;

fn main() {
    for range in Ranges::new(read_integers()) {
        print!("{{{}..{}}} ", range.start(), range.end());
    }

    println!();
}

fn read_integers() -> impl Iterator<Item = i64> {
    stdin()
        .lines()
        .take_while(|line| line.is_ok())
        .map(|line| line.unwrap().parse::<i64>())
        .take_while(|result| result.is_ok())
        .map(|result| result.unwrap())
}
