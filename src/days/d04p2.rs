use aoc::utils::BufferedInput;
use std::io::{BufRead, Error, ErrorKind};

use itertools::Itertools;

fn parse_input() -> std::io::Result<(i32, i32)> {
    let input = BufferedInput::parse_args("Day 4: Secure Container - Part 2")?;
    let line = input
        .lines()
        .map(Result::unwrap)
        .next()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Input has no content"))?;

    let result = line
        .as_str()
        .split('-')
        .map(|n| n.parse().expect("Failed to parse password check bounds"))
        .collect_tuple()
        .unwrap();

    Ok(result)
}

fn is_valid(password: &str) -> bool {
    let mut group_len = 1;
    let mut is_adjacent_pair_same = false;

    for (a, b) in password.chars().tuple_windows() {
        if a > b {
            return false;
        }

        if a == b {
            group_len += 1;
        } else {
            if group_len == 2 {
                is_adjacent_pair_same = true;
            }

            group_len = 1;
        }
    }

    group_len == 2 || is_adjacent_pair_same
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
