mod io;

use itertools::{izip, Itertools};
use std::cmp::{PartialEq, PartialOrd};
use std::ops::{Add, Mul};

pub use io::{IoProvider, ValueProvider};

#[derive(Debug)]
pub struct Machine<'a, T> {
    memory: Vec<i64>,
    program_counter: usize,
    relative_base: i64,
    jump_flag: Option<usize>,
    halted: bool,
    running: bool,
    interrupt_on_output: bool,
    output_interrupted_flag: bool,
    last_output: Option<i64>,
    io_provider: &'a mut T,
}

#[derive(Copy, Clone)]
enum AddressMode {
    Position,
    Immediate,
    Relative,
}

type Arg = (AddressMode, i64);

#[derive(Copy, Clone)]
enum Args {
    Zero,
    One(Arg),
    Two(Arg, Arg),
    Three(Arg, Arg, Arg),
}

fn get_modes(mode_num: i64) -> impl Iterator<Item = AddressMode> {
    itertools::unfold(mode_num, |state| {
        let current = *state % 10;
        *state /= 10;

        let result = match current {
            0 => AddressMode::Position,
            1 => AddressMode::Immediate,
            2 => AddressMode::Relative,
            _ => panic!("Error when parsing mode: unrecognized address mode"),
        };

        Some(result)
    })
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
            panic!(msg);
        }
    }

    pub fn expect_one(self, msg: &'static str) -> Arg {
        match self {
            Args::One(arg) => arg,
            _ => panic!(msg),
        }
    }

    pub fn expect_two(self, msg: &'static str) -> (Arg, Arg) {
        match self {
            Args::Two(a, b) => (a, b),
            _ => panic!(msg),
        }
    }

    pub fn expect_three(self, msg: &'static str) -> (Arg, Arg, Arg) {
        match self {
            Args::Three(a, b, c) => (a, b, c),
            _ => panic!(msg),
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

    pub fn set_interrupt_on_output(&mut self, switch: bool) {
        self.interrupt_on_output = switch;
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
            interrupt_on_output: false,
            output_interrupted_flag: false,
            last_output: None,
            io_provider,
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

    fn exec(&mut self, opcode: i64, args: Args) {
        match opcode {
            1 => self.arithmetic_operation(args, Add::add),
            2 => self.arithmetic_operation(args, Mul::mul),
            3 => self.input_operation(args),
            4 => self.output_operation(args),
            5 => self.jump_operation(args, |v| v != 0),
            6 => self.jump_operation(args, |v| v == 0),
            7 => self.compare_operation(args, PartialOrd::lt),
            8 => self.compare_operation(args, PartialEq::eq),
            9 => self.relative_base_operation(args),
            99 => self.terminate(args),
            _ => panic!("Error during execution: unrecognized opcode"),
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

    fn parse_instruction(&self) -> (i64, Args) {
        let opcode_unparsed = self.memory[self.program_counter];
        let op = opcode_unparsed % 100;

        let n_args = match op {
            1 | 2 | 7 | 8 => 3,
            5 | 6 => 2,
            3 | 4 | 9 => 1,
            99 => 0,
            _ => panic!("Error when parsing instruction: unrecognized opcode"),
        };

        (op, self.get_args(n_args, opcode_unparsed / 100))
    }

    fn get_args(&self, n_args: usize, modes: i64) -> Args {
        let arg_begin = self.program_counter + 1;
        let arg_end = self.program_counter + n_args + 1;
        let modes = get_modes(modes);
        let mem_slice = self.memory[arg_begin..arg_end].iter().copied();
        let args = izip!(modes, mem_slice);

        match n_args {
            3 => {
                let (a, b, c) = args.collect_tuple().unwrap();
                Args::Three(a, b, c)
            }
            2 => {
                let (a, b) = args.collect_tuple().unwrap();
                Args::Two(a, b)
            }
            1 => {
                let (a,) = args.collect_tuple().unwrap();
                Args::One(a)
            }
            0 => Args::Zero,
            _ => panic!("Error: unrecognized argument number"),
        }
    }

    fn get_value_from_arg(&mut self, (mode, v): Arg) -> i64 {
        match mode {
            AddressMode::Position => self.try_read_or_resize(v as usize),
            AddressMode::Immediate => v,
            AddressMode::Relative => {
                let relative_address = self.relative_base + v;

                self.try_read_or_resize(relative_address as usize)
            }
        }
    }

    fn get_address_from_arg(&self, (mode, v): Arg) -> usize {
        match mode {
            AddressMode::Position => v as usize,
            AddressMode::Relative => (self.relative_base + v) as usize,
            AddressMode::Immediate => panic!("Error: write access at an address in immediate mode"),
        }
    }

    fn try_read_or_resize(&mut self, position: usize) -> i64 {
        let current_len = self.memory.len();
        if position >= current_len {
            let extension_size = position - current_len + 1;

            self.extend_by(extension_size);
        }

        self.memory[position]
    }

    fn try_write_or_resize(&mut self, position: usize, value: i64) {
        let current_len = self.memory.len();
        if position >= current_len {
            let extension_size = position - current_len + 1;

            self.extend_by(extension_size);
        }

        self.memory[position] = value;
    }

    fn extend_by(&mut self, number: usize) {
        let extender = std::iter::repeat(0).take(number);

        self.memory.extend(extender);
    }
}
