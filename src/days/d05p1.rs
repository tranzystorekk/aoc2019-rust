use aoc::intcode::{Machine, ValueProvider};
use aoc::utils::parse_intcode_program;

fn main() -> std::io::Result<()> {
    let program = parse_intcode_program("Day 5: Sunny with a Chance of Asteroids - Part 1")?;

    let ref mut io = ValueProvider::new(1);
    let mut machine = Machine::new(program, io);
    machine.run();

    let code = machine.last_output().unwrap();

    println!("{}", code);

    Ok(())
}
