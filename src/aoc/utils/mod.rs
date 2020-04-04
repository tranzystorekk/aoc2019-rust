pub mod args;
pub mod bufwrap;

use std::io::{BufRead, Error, ErrorKind};

pub use args::BufferedInput;

pub fn parse_intcode_program(description: &str) -> std::io::Result<Vec<i64>> {
    let input = BufferedInput::parse_args(description)?;
    let line = input
        .lines()
        .map(Result::unwrap)
        .next()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Input has no content"))?;

    let result = line
        .as_str()
        .split(',')
        .map(|s| s.parse().expect("Failed to parse intcode program"))
        .collect();

    Ok(result)
}
