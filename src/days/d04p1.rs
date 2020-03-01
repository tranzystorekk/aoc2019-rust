use std::io::{BufRead, Error, ErrorKind};
use utils::args::BufferedInput;

use itertools::Itertools;

fn parse_input() -> std::io::Result<(i32, i32)> {
    let input = BufferedInput::parse_args("Day 4: Secure Container - Part 1")?;
    let line = input.lines()
        .map(Result::unwrap)
        .next()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Input has no content"))?;

    let result = line.as_str()
        .split('-')
        .map(|n| n.parse().unwrap())
        .collect_tuple()
        .unwrap();

    Ok(result)
}

fn is_valid(password: &str) -> bool {
    let mut is_adjacent_same = false;

    for (a, b) in password.chars().tuple_windows() {
        if a > b {
            return false;
        }

        if a == b {
            is_adjacent_same = true;
        }
    }

    is_adjacent_same
}

fn main() -> std::io::Result<()> {
    let (lower, upper) = parse_input()?;

    let result = (lower..=upper)
        .map(|n| n.to_string())
        .filter(|pass| is_valid(pass.as_str()))
        .count();

    println!("{}", result);

    Ok(())
}
