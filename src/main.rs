use ranges::Ranges;
use std::io::stdin;
use std::ops::RangeInclusive;

fn main() {
    println!(
        "{}",
        Ranges::from(read_integers())
            .map(range_to_string)
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn range_to_string(range: RangeInclusive<i64>) -> String {
    format!("{{{}..{}}}", range.start(), range.end())
}

fn read_integers() -> impl Iterator<Item = i64> {
    stdin()
        .lines()
        .take_while(|line| line.is_ok())
        .map(|line| line.unwrap())
        .flat_map(|line| {
            line.split_ascii_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .map(|line| line.parse::<i64>())
        .take_while(|result| result.is_ok())
        .map(|result| result.unwrap())
}
