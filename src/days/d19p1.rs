#[macro_use]
extern crate itertools;

use aoc::intcode::{IoProvider, Machine};
use aoc::utils::parse_intcode_program;

struct PosChecker {
    positions: std::vec::IntoIter<i64>,
    result: Option<i64>,
}

impl PosChecker {
    pub fn from_pos((x, y): (i64, i64)) -> Self {
        Self {
            positions: vec![x, y].into_iter(),
            result: None,
        }
    }

    pub fn get_result(&self) -> Option<i64> {
        self.result
    }
}

impl IoProvider for PosChecker {
    fn send_input(&mut self) -> i64 {
        self.positions.next().unwrap()
    }

    fn get_output(&mut self, value: i64) {
        self.result = Some(value)
    }
}

fn main() -> std::io::Result<()> {
    let program = parse_intcode_program("Day 19: Tractor Beam - Part 1")?;

    let result: i64 = iproduct!(0..50, 0..50)
        .map(|pos| {
            let ref mut checker = PosChecker::from_pos(pos);
            let mut cpu = Machine::new(program.clone(), checker);

            cpu.run();
            checker.get_result().unwrap()
        })
        .sum();

    println!("{}", result);
    Ok(())
}
