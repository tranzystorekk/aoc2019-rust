use std::io::{BufRead, Error, ErrorKind};

use itertools::Itertools;

use aoc::intcode::{IoProvider, Machine};
use aoc::utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<i64>> {
    let input = BufferedInput::parse_args("Day 7: Amplification Circuit - Part 1")?;
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

struct Input {
    it: Box<dyn Iterator<Item = i64>>,
}

impl Input {
    pub fn with_params(phase: i64, input: i64) -> Self {
        let seq = vec![phase, input];
        Input {
            it: Box::new(seq.into_iter())
        }
    }
}

impl IoProvider for Input {
    fn send_input(& mut self) -> i64 {
        self.it.next().unwrap()
    }
}

fn run_series(phases: Vec<i64>, prog: &Vec<i64>) -> i64 {
    let mut current_value = 0;

    for phase in phases {
        let ref mut inp = Input::with_params(phase, current_value);
        let mut cpu = Machine::new(prog.clone(), inp);
        cpu.run();

        current_value = cpu.last_output().unwrap();
    }

    current_value
}

fn main() -> std::io::Result<()> {
    let ref program = parse_input()?;

    let result = (0..5).permutations(5)
        .map(|ph| run_series(ph, program))
        .max()
        .unwrap();

    println!("{}", result);

    Ok(())
}
