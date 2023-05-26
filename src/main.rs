use ranges::Ranges;
use std::fmt::{Debug, Display};
use std::io::stdin;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() {
    println!(
        "{}",
        read_integers::<i64>()
            .ranges()
            .map(range_to_bash_literal)
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn read_integers<T>() -> impl Iterator<Item = T>
where
    T: FromStr,
    T::Err: Debug,
{
    stdin()
        .lines()
        .take_while(|line| line.is_ok())
        .map(|line| line.unwrap())
        .flat_map(|line| {
            line.split_ascii_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .map(|number| number.parse::<T>())
        .take_while(|result| result.is_ok())
        .map(|result| result.unwrap())
}

fn range_to_bash_literal<T>(range: RangeInclusive<T>) -> String
where
    T: Display + PartialEq,
{
    if range.start() == range.end() {
        range.start().to_string()
    } else {
        format!("{{{}..{}}}", range.start(), range.end())
    }
}
