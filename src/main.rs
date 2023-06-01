use ranges::Ranges;
use std::fmt::Debug;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::str::FromStr;

fn main() {
    let mut stdout = BufWriter::new(stdout().lock());
    let mut separator = "";

    read_integers::<i64>().ranges().for_each(|range| {
        write!(stdout, "{separator}{range}").expect("Could not write to STDOUT.");
        separator = " ";
    });
    println!();
}

fn read_integers<T>() -> impl Iterator<Item = T>
where
    T: FromStr,
    T::Err: Debug,
{
    BufReader::new(stdin().lock())
        .lines()
        .map_while(|line| line.ok())
        .flat_map(|line| {
            line.split_ascii_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .map_while(|number| number.parse::<T>().ok())
}
