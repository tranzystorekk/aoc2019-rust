use aoc::intcode::{IoProvider, Machine};
use aoc::utils::parse_intcode_program;

enum InstructionState {
    AwaitingX,
    AwaitingY,
    AwaitingId,
}

struct Arcade {
    n_blocks: usize,
    state: InstructionState,
}

impl InstructionState {
    pub fn advance(&mut self) {
        *self = match self {
            InstructionState::AwaitingX => InstructionState::AwaitingY,
            InstructionState::AwaitingY => InstructionState::AwaitingId,
            InstructionState::AwaitingId => InstructionState::AwaitingX,
        };
    }
}

impl Arcade {
    pub fn new() -> Self {
        Arcade {
            n_blocks: 0,
            state: InstructionState::AwaitingX,
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
        if let InstructionState::AwaitingId = self.state {
            if value == 2 {
                self.n_blocks += 1;
            }
        }

        self.state.advance();
    }
}

fn main() -> std::io::Result<()> {
    let program = parse_intcode_program("Day 13: Care Package - Part 1")?;

    let ref mut arcade = Arcade::new();
    let mut cpu = Machine::new(program, arcade);
    cpu.run();

    println!("{}", arcade.n_blocks());

    Ok(())
}
