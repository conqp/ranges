use ranges::Ranges;
use std::fmt::Debug;
use std::io::{stdin, stdout, BufWriter, Write};
use std::str::FromStr;

fn main() {
    let mut stdout = BufWriter::new(stdout().lock());
    let mut separator = "";

    read_integers::<i64>().ranges().for_each(|range| {
        write!(stdout, "{separator}{range}").expect("Could not write to STDOUT.");
        separator = " ";
    })
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
