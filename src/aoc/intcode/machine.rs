use itertools::{izip, Itertools};
use std::cmp::{PartialEq, PartialOrd};
use std::ops::{Add as Addition, Mul as Multiply};

use super::io::IoProvider;

#[derive(Debug)]
pub struct Machine<'a, T> {
    memory: Vec<i64>,
    program_counter: usize,
    relative_base: i64,
    jump_flag: Option<usize>,
    halted: bool,
    running: bool,
    output_interrupted_flag: bool,
    last_output: Option<i64>,
    io_provider: &'a mut T,
    pub interrupt_on_output: bool,
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Mul,
    Inp,
    Out,
    Jnz,
    Jez,
    Tlt,
    Teq,
    Rel,
    Hlt,
}

#[derive(Copy, Clone)]
enum Arg {
    Position(i64),
    Immediate(i64),
    Relative(i64),
}

#[derive(Copy, Clone)]
enum Args {
    Zero,
    One(Arg),
    Two(Arg, Arg),
    Three(Arg, Arg, Arg),
}

fn get_modes(mode_num: i64) -> impl Iterator<Item = i64> {
    itertools::unfold(mode_num, |state| {
        let current = *state % 10;
        *state /= 10;

        Some(current)
    })
}

impl Op {
    pub fn from_code(code: i64) -> Self {
        use Op::*;

        match code {
            1 => Add,
            2 => Mul,
            3 => Inp,
            4 => Out,
            5 => Jnz,
            6 => Jez,
            7 => Tlt,
            8 => Teq,
            9 => Rel,
            99 => Hlt,
            _ => panic!("Error when parsing opcode: unrecognized opcode"),
        }
    }

    pub fn expected_n_args(&self) -> usize {
        use Op::*;

        match self {
            Add | Mul | Tlt | Teq => 3,
            Jnz | Jez => 2,
            Inp | Out | Rel => 1,
            Hlt => 0,
        }
    }
}

impl Arg {
    pub fn from_tuple((mode, v): (i64, i64)) -> Self {
        match mode {
            0 => Arg::Position(v),
            1 => Arg::Immediate(v),
            2 => Arg::Relative(v),
            _ => panic!("Error when parsing mode: unrecognized address mode"),
        }
    }
}

impl Args {
    pub fn len(&self) -> usize {
        match self {
            Args::Three(_, _, _) => 3,
            Args::Two(_, _) => 2,
            Args::One(_) => 1,
            _ => 0,
        }
    }

    pub fn expect_zero(&self, msg: &'static str) {
        if !matches!(self, Args::Zero) {
            panic!("{}", msg);
        }
    }

    pub fn expect_one(self, msg: &'static str) -> Arg {
        match self {
            Args::One(arg) => arg,
            _ => panic!("{}", msg),
        }
    }

    pub fn expect_two(self, msg: &'static str) -> (Arg, Arg) {
        match self {
            Args::Two(a, b) => (a, b),
            _ => panic!("{}", msg),
        }
    }

    pub fn expect_three(self, msg: &'static str) -> (Arg, Arg, Arg) {
        match self {
            Args::Three(a, b, c) => (a, b, c),
            _ => panic!("{}", msg),
        }
    }
}

impl<T> Machine<'_, T> {
    pub fn read(&self, position: usize) -> i64 {
        self.memory[position]
    }

    pub fn write(&mut self, position: usize, value: i64) {
        self.memory[position] = value;
    }

    pub fn last_output(&self) -> Option<i64> {
        self.last_output
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn provider(&self) -> &T {
        self.io_provider
    }

    pub fn provider_mut(&mut self) -> &mut T {
        self.io_provider
    }
}

impl<'a, T: IoProvider> Machine<'a, T> {
    pub fn new(program: Vec<i64>, io_provider: &'a mut T) -> Self {
        Machine::<'a, T> {
            memory: program,
            program_counter: 0,
            relative_base: 0,
            jump_flag: None,
            halted: false,
            running: false,
            output_interrupted_flag: false,
            last_output: None,
            io_provider,
            interrupt_on_output: false,
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.run_until_interrupt();
        }
    }

    pub fn step(&mut self) {
        if !self.halted {
            self.step_internal();
            self.check_flags();
        }
    }

    pub fn run_until_interrupt(&mut self) {
        self.running = true;
        loop {
            self.step_internal();
            self.check_flags();

            if !self.running {
                break;
            }
        }
    }

    fn step_internal(&mut self) {
        let (opcode, args) = self.parse_instruction();
        self.exec(opcode, args);

        self.program_counter = self
            .jump_flag
            .take()
            .unwrap_or_else(|| self.program_counter + args.len() + 1);
    }

    fn check_flags(&mut self) {
        if self.output_interrupted_flag {
            self.running = false;
            self.output_interrupted_flag = false;
        }
    }

    fn exec(&mut self, opcode: Op, args: Args) {
        use Op::*;

        match opcode {
            Add => self.arithmetic_operation(args, Addition::add),
            Mul => self.arithmetic_operation(args, Multiply::mul),
            Inp => self.input_operation(args),
            Out => self.output_operation(args),
            Jnz => self.jump_operation(args, |v| v != 0),
            Jez => self.jump_operation(args, |v| v == 0),
            Tlt => self.compare_operation(args, PartialOrd::lt),
            Teq => self.compare_operation(args, PartialEq::eq),
            Rel => self.relative_base_operation(args),
            Hlt => self.terminate(args),
        };
    }

    fn terminate(&mut self, args: Args) {
        args.expect_zero("Error: terminate received non-empty arguments");

        self.running = false;
        self.halted = true;
    }

    fn arithmetic_operation<F: FnOnce(i64, i64) -> i64>(&mut self, args: Args, op: F) {
        let (arg_a, arg_b, arg_dest) =
            args.expect_three("Error: invalid arguments for comparison operation");
        let a = self.get_value_from_arg(arg_a);
        let b = self.get_value_from_arg(arg_b);
        let dest_addr = self.get_address_from_arg(arg_dest);

        let result = op(a, b);
        self.try_write_or_resize(dest_addr, result);
    }

    fn compare_operation<F: FnOnce(&i64, &i64) -> bool>(&mut self, args: Args, op: F) {
        let (arg_a, arg_b, arg_dest) =
            args.expect_three("Error: invalid arguments for comparison operation");
        let a = &self.get_value_from_arg(arg_a);
        let b = &self.get_value_from_arg(arg_b);
        let dest_addr = self.get_address_from_arg(arg_dest);

        let result = op(a, b) as i64;
        self.try_write_or_resize(dest_addr, result);
    }

    fn jump_operation<F: FnOnce(i64) -> bool>(&mut self, args: Args, op: F) {
        let (arg_cond, arg_addr) = args.expect_two("Error: invalid arguments for jump operation");
        let cond = self.get_value_from_arg(arg_cond);
        let addr = self.get_value_from_arg(arg_addr) as usize;

        if op(cond) {
            self.jump_flag.replace(addr);
        }
    }

    fn relative_base_operation(&mut self, args: Args) {
        let arg_offset = args.expect_one("Error: invalid arguments for relative base operation");
        let offset = self.get_value_from_arg(arg_offset);

        self.relative_base += offset;
    }

    fn input_operation(&mut self, args: Args) {
        let arg_addr = args.expect_one("Error: invalid arguments for input operation");
        let addr = self.get_address_from_arg(arg_addr);

        let input_value = self.io_provider.send_input();
        self.try_write_or_resize(addr, input_value);
    }

    fn output_operation(&mut self, args: Args) {
        let arg_value = args.expect_one("Error: invalid arguments for output operation");
        let output_value = self.get_value_from_arg(arg_value);

        self.io_provider.get_output(output_value);
        self.last_output = Some(output_value);

        if self.interrupt_on_output {
            self.output_interrupted_flag = true;
        }
    }

    fn parse_instruction(&self) -> (Op, Args) {
        let opcode_unparsed = self.memory[self.program_counter];
        let code = opcode_unparsed % 100;

        let operation = Op::from_code(code);
        let n_args = operation.expected_n_args();
        let args = self.get_args(n_args, opcode_unparsed / 100);
        (operation, args)
    }

    fn get_args(&self, n_args: usize, modes: i64) -> Args {
        let arg_begin = self.program_counter + 1;
        let modes = get_modes(modes);
        let mem_slice = self.memory[arg_begin..].iter().copied();
        let mut args = izip!(modes, mem_slice).map(Arg::from_tuple);

        match n_args {
            3 => {
                let (a, b, c) = args.next_tuple().unwrap();
                Args::Three(a, b, c)
            }
            2 => {
                let (a, b) = args.next_tuple().unwrap();
                Args::Two(a, b)
            }
            1 => {
                let (a,) = args.next_tuple().unwrap();
                Args::One(a)
            }
            0 => Args::Zero,
            _ => panic!("Error: unrecognized argument number"),
        }
    }

    fn get_value_from_arg(&mut self, arg: Arg) -> i64 {
        match arg {
            Arg::Position(v) => self.try_read_or_resize(v as usize),
            Arg::Immediate(v) => v,
            Arg::Relative(v) => {
                let relative_address = self.relative_base + v;
                self.try_read_or_resize(relative_address as usize)
            }
        }
    }

    fn get_address_from_arg(&self, arg: Arg) -> usize {
        match arg {
            Arg::Position(v) => v as usize,
            Arg::Relative(v) => (self.relative_base + v) as usize,
            Arg::Immediate(_) => panic!("Error: write access at an address in immediate mode"),
        }
    }

    fn try_read_or_resize(&mut self, position: usize) -> i64 {
        self.resize_if_needed(position);
        self.memory[position]
    }

    fn try_write_or_resize(&mut self, position: usize, value: i64) {
        self.resize_if_needed(position);
        self.memory[position] = value;
    }

    fn resize_if_needed(&mut self, position: usize) {
        let current_len = self.memory.len();
        if position >= current_len {
            let extension_size = position - current_len + 1;
            self.extend_by(extension_size);
        }
    }

    fn extend_by(&mut self, number: usize) {
        let extender = std::iter::repeat(0).take(number);

        self.memory.extend(extender);
    }
}
