use std::io::{BufRead, Error, ErrorKind};

use itertools::Itertools;

use aoc::intcode::{Machine, ValueProvider};
use aoc::utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<i64>> {
    let input = BufferedInput::parse_args("Day 2: 1202 Program Alarm - Part 2")?;
    let line = input.lines()
        .map(Result::unwrap)
        .next()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Input has no content"))?;

    let result = line.as_str()
        .split(',')
        .map(|s| s.parse().expect("Failed to parse intcode program"))
        .collect();

    Ok(result)
}

fn main() -> std::io::Result<()> {
    let program = parse_input()?;
    let ref mut io = ValueProvider::new(0);

    for (noun, verb) in (0..100).cartesian_product(0..100) {
        let mut machine = Machine::new(program.clone(), io);
        machine.write(1, noun);
        machine.write(2, verb);

        machine.run();

        let output = machine.read(0);

        if output == 19690720 {
            let result = 100 * noun + verb;
            println!("{}", result);

            break;
        }
    }

    Ok(())
}
