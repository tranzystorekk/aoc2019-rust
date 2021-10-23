use itertools::Itertools;

use aoc::intcode::{IoProvider, Machine};
use aoc::utils::parse_intcode_program;

struct Input {
    it: std::array::IntoIter<i64, 2>,
}

impl Input {
    pub fn with_params(phase: i64, input: i64) -> Self {
        Input {
            it: [phase, input].into_iter(),
        }
    }
}

impl IoProvider for Input {
    fn send_input(&mut self) -> i64 {
        self.it.next().unwrap()
    }
}

fn run_series(phases: Vec<i64>, prog: &[i64]) -> i64 {
    let mut current_value = 0;

    for phase in phases {
        let inp = &mut Input::with_params(phase, current_value);
        let mut cpu = Machine::new(prog.into(), inp);
        cpu.run();

        current_value = cpu.last_output().unwrap();
    }

    current_value
}

fn main() -> std::io::Result<()> {
    let program = &(parse_intcode_program("Day 7: Amplification Circuit - Part 1")?);

    let result = (0..5)
        .permutations(5)
        .map(|ph| run_series(ph, program))
        .max()
        .unwrap();

    println!("{}", result);

    Ok(())
}
