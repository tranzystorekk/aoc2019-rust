use aoc::intcode::{Machine, ValueProvider};
use aoc::utils::parse_intcode_program;

fn main() -> std::io::Result<()> {
    let program = parse_intcode_program("Day 2: 1202 Program Alarm - Part 1")?;

    let io = &mut ValueProvider::new(0);
    let mut machine = Machine::new(program, io);
    machine.write(1, 12);
    machine.write(2, 2);

    machine.run();

    let zero_pos = machine.read(0);

    println!("{}", zero_pos);

    Ok(())
}
