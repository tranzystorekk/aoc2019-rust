use std::io::{BufRead, Error, ErrorKind};
use aoc::utils::BufferedInput;

fn parse_input() -> std::io::Result<(String, Vec<i32>)> {
    let input = BufferedInput::parse_args("Day 16: Flawed Frequency Transmission - Part 2")?;
    let line = input.lines()
        .map(Result::unwrap)
        .next()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Input has no content"))?;

    let result = line.as_str()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    Ok((line, result))
}

fn compute_phase(data: Vec<i32>) -> Vec<i32> {
    let mut transformed: Vec<i32> = data.into_iter().rev()
        .scan(0, |sum, val| {
        *sum += val;

        Some(*sum % 10)
    })
        .collect();

    transformed.reverse();

    transformed
}

fn run_phases(initial: Vec<i32>, n: usize) -> Vec<i32> {
    std::iter::successors(Some(initial), |data| Some(compute_phase(data.clone())))
        .take(n+1)
        .last()
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let (line, received_data) = parse_input()?;
    let message_offset: usize = line[..7]
        .parse()
        .unwrap();

    let real_data: Vec<i32> = std::iter::repeat(received_data)
        .take(10000)
        .flat_map(|data| data.into_iter())
        .skip(message_offset)
        .collect();

    let computed = run_phases(real_data, 100);

    let code: String = computed[..8].iter()
        .map(|digit| std::char::from_digit(*digit as u32, 10).unwrap())
        .collect();

    println!("{}", code);

    Ok(())
}
