use std::io::{BufRead, Error, ErrorKind};

use aoc::intcode::Machine;
use aoc::utils::args::BufferedInput;

fn parse_input() -> std::io::Result<Vec<i64>> {
    let input = BufferedInput::parse_args("Day 2: 1202 Program Alarm - Part 1")?;
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

    let mut machine = Machine::new(program, || 0, |_| ());
    machine.write(1, 12);
    machine.write(2, 2);

    machine.run();

    let zero_pos = machine.read(0);

    println!("{}", zero_pos);

    Ok(())
}
