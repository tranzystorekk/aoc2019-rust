use itertools::Itertools;

use aoc::intcode::{IoProvider, Machine};
use aoc::utils::parse_intcode_program;

struct Link {
    phase: i64,
    value: i64,
    awaiting_phase: bool,
}

impl Link {
    pub fn with_phase(phase: i64) -> Self {
        Link {
            phase,
            value: 0,
            awaiting_phase: true,
        }
    }

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn set_value(&mut self, v: i64) {
        self.value = v;
    }
}

impl IoProvider for Link {
    fn send_input(&mut self) -> i64 {
        if self.awaiting_phase {
            self.awaiting_phase = false;
            self.phase
        } else {
            self.value
        }
    }

    fn get_output(&mut self, value: i64) {
        self.value = value;
    }
}

fn run_feedback_loop(phases: Vec<i64>, prog: &[i64]) -> i64 {
    let mut links: Vec<Link> = phases.into_iter().map(Link::with_phase).collect();
    let mut cpus: Vec<_> = links
        .iter_mut()
        .map(|link| {
            let mut m = Machine::new(prog.into(), link);
            m.interrupt_on_output = true;

            m
        })
        .collect();

    let mut current_value = 0;
    while !cpus.iter().all(|m| m.is_halted()) {
        for cpu in cpus.iter_mut() {
            cpu.provider_mut().set_value(current_value);
            cpu.run_until_interrupt();
            current_value = cpu.provider().value();
        }
    }

    current_value
}

fn main() -> std::io::Result<()> {
    let program = &(parse_intcode_program("Day 7: Amplification Circuit - Part 2")?);

    let result = (5..10)
        .permutations(5)
        .map(|ph| run_feedback_loop(ph, program))
        .max()
        .unwrap();

    println!("{}", result);

    Ok(())
}
