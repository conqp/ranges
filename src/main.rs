use ranges::Ranges;
use std::io::stdin;
use std::ops::RangeInclusive;

fn main() {
    println!(
        "{}",
        read_integers()
            .ranges()
            .map(range_to_bash_literal)
            .collect::<Vec<_>>()
            .join(" ")
    )
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
        .map(|number| number.parse::<i64>())
        .take_while(|result| result.is_ok())
        .map(|result| result.unwrap())
}

fn range_to_bash_literal(range: RangeInclusive<i64>) -> String {
    if range.start() == range.end() {
        range.start().to_string()
    } else {
        format!("{{{}..{}}}", range.start(), range.end())
    }
}
