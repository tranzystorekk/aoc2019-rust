use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

use aoc::intcode::{IoProvider, Machine};
use aoc::utils::parse_intcode_program;

#[macro_use]
extern crate maplit;

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

enum Direction {
    North,
    East,
    South,
    West,
}

enum InstructionState {
    AwaitingColor,
    AwaitingTurn,
}

struct Walker {
    x: i64,
    y: i64,
    facing: Direction,
}

impl Color {
    pub fn from_value(val: i64) -> Self {
        match val {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Invalid color value"),
        }
    }

    pub fn value(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl Direction {
    pub fn turn_left(&mut self) {
        *self = match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl InstructionState {
    pub fn advance(&mut self) {
        *self = match self {
            InstructionState::AwaitingColor => InstructionState::AwaitingTurn,
            InstructionState::AwaitingTurn => InstructionState::AwaitingColor,
        }
    }
}

impl Walker {
    pub fn new() -> Self {
        Walker {
            x: 0,
            y: 0,
            facing: Direction::North,
        }
    }

    pub fn walk(&mut self) {
        match self.facing {
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1,
        }
    }

    pub fn turn_left(&mut self) {
        self.facing.turn_left();
    }

    pub fn turn_right(&mut self) {
        self.facing.turn_right();
    }

    pub fn position(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

struct PainterBot {
    grid: HashMap<(i64, i64), Color>,
    walker: Walker,
    state: InstructionState,
}

impl PainterBot {
    pub fn new() -> Self {
        PainterBot {
            grid: hashmap! { (0, 0) => Color::White },
            walker: Walker::new(),
            state: InstructionState::AwaitingColor,
        }
    }

    pub fn grid(&self) -> &HashMap<(i64, i64), Color> {
        &self.grid
    }

    fn current_color(&self) -> Color {
        let ref pos = self.walker.position();

        self.grid.get(pos).copied().unwrap_or(Color::Black)
    }

    fn read_instruction(&mut self, instr: i64) {
        match self.state {
            InstructionState::AwaitingColor => {
                let pos = self.walker.position();

                self.grid.insert(pos, Color::from_value(instr));
            }
            InstructionState::AwaitingTurn => {
                match instr {
                    0 => self.walker.turn_left(),
                    1 => self.walker.turn_right(),
                    _ => panic!("Invalid direction instruction from CPU"),
                };

                self.walker.walk();
            }
        };

        self.state.advance();
    }
}

impl IoProvider for PainterBot {
    fn send_input(&mut self) -> i64 {
        self.current_color().value()
    }

    fn get_output(&mut self, value: i64) {
        self.read_instruction(value);
    }
}

fn main() -> std::io::Result<()> {
    let program = parse_intcode_program("Day 11: Space Police - Part 2")?;

    let ref mut bot = PainterBot::new();
    let mut cpu = Machine::new(program, bot);
    cpu.run();

    let white_tiles: HashSet<(i64, i64)> = bot
        .grid()
        .iter()
        .filter_map(|(pos, color)| match color {
            Color::White => Some(*pos),
            _ => None,
        })
        .collect();

    let (min_x, max_x) = match white_tiles.iter().minmax_by_key(|(x, _)| x) {
        MinMax(&(min_x, _), &(max_x, _)) => (min_x, max_x),
        _ => panic!("Incorrect picture"),
    };

    let (min_y, max_y) = match white_tiles.iter().minmax_by_key(|(_, y)| y) {
        MinMax(&(_, min_y), &(_, max_y)) => (min_y, max_y),
        _ => panic!("Incorrect picture"),
    };

    let picture = (min_y..max_y + 1)
        .rev()
        .map(|y| {
            (min_x..max_x + 1)
                .map(|x| {
                    if white_tiles.contains(&(x, y)) {
                        '#'
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        })
        .join("\n");

    println!("{}", picture);

    Ok(())
}
