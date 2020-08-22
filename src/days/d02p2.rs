use itertools::Itertools;

use aoc::intcode::{Machine, ValueProvider};
use aoc::utils::parse_intcode_program;

fn main() -> std::io::Result<()> {
    let program = parse_intcode_program("Day 2: 1202 Program Alarm - Part 2")?;
    let io = &mut ValueProvider::new(0);

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
