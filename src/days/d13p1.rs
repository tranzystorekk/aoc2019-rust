use aoc::intcode::{IoProvider, Machine};
use aoc::utils::parse_intcode_program;

enum InstructionState {
    X,
    Y,
    Id,
}

struct Arcade {
    n_blocks: usize,
    state: InstructionState,
}

impl InstructionState {
    pub fn advance(&mut self) {
        *self = match self {
            InstructionState::X => InstructionState::Y,
            InstructionState::Y => InstructionState::Id,
            InstructionState::Id => InstructionState::X,
        };
    }
}

impl Arcade {
    pub fn new() -> Self {
        Arcade {
            n_blocks: 0,
            state: InstructionState::X,
        }
    }

    pub fn n_blocks(&self) -> usize {
        self.n_blocks
    }
}

impl IoProvider for Arcade {
    fn send_input(&mut self) -> i64 {
        0
    }

    fn get_output(&mut self, value: i64) {
        if let InstructionState::Id = self.state {
            if value == 2 {
                self.n_blocks += 1;
            }
        }

        self.state.advance();
    }
}

fn main() -> std::io::Result<()> {
    let program = parse_intcode_program("Day 13: Care Package - Part 1")?;

    let arcade = &mut Arcade::new();
    let mut cpu = Machine::new(program, arcade);
    cpu.run();

    println!("{}", arcade.n_blocks());

    Ok(())
}
