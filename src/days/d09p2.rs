use std::io::{BufRead, Error, ErrorKind};

use aoc::intcode::{Machine, ValueProvider};
use aoc::utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<i64>> {
    let input = BufferedInput::parse_args("Day 9: Sensor Boost - Part 2")?;
    let line = input.lines()
        .map(Result::unwrap)
        .next()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Input has no content"))?;

    let result = line.as_str()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    Ok(result)
}

fn main() -> std::io::Result<()> {
    let program = parse_input()?;

    let ref mut io = ValueProvider::new(2);
    let mut machine = Machine::new(program, io);
    machine.run();

    let code = machine.last_output().unwrap();

    println!("{}", code);

    Ok(())
}
