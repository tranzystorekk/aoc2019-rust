use aoc::intcode::{IoProvider, Machine};
use aoc::utils::parse_intcode_program;
use std::cmp::Ordering;

enum InstructionState {
    AwaitingX,
    AwaitingY,
    AwaitingId,
}

struct Arcade {
    current_x: i64,
    current_y: i64,
    paddle_x: i64,
    ball_x: i64,
    score: i64,
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
            current_x: 0,
            current_y: 0,
            paddle_x: 0,
            ball_x: 0,
            score: 0,
            state: InstructionState::AwaitingX,
        }
    }

    pub fn score(&self) -> i64 {
        self.score
    }

    fn process_tile(&mut self, x: i64, id: i64) {
        match id {
            3 => self.paddle_x = x,
            4 => self.ball_x = x,
            _ => (),
        }
    }
}

impl IoProvider for Arcade {
    fn send_input(&mut self) -> i64 {
        match Ord::cmp(&self.paddle_x, &self.ball_x) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        }
    }

    fn get_output(&mut self, value: i64) {
        match self.state {
            InstructionState::AwaitingX => self.current_x = value,
            InstructionState::AwaitingY => self.current_y = value,
            InstructionState::AwaitingId => match (self.current_x, self.current_y) {
                (-1, 0) => self.score = value,
                (x, _) => self.process_tile(x, value),
            },
        }

        self.state.advance();
    }
}

fn main() -> std::io::Result<()> {
    let program = parse_intcode_program("Day 13: Care Package - Part 2")?;

    let ref mut arcade = Arcade::new();
    let mut cpu = Machine::new(program, arcade);
    cpu.write(0, 2);
    cpu.run();

    println!("{}", arcade.score());

    Ok(())
}
