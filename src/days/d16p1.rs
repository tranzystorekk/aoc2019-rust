use aoc::utils::BufferedInput;
use std::io::{Error, ErrorKind};

use itertools::iterate;

fn parse_input() -> std::io::Result<Vec<i32>> {
    let input = BufferedInput::parse_args("Day 16: Flawed Frequency Transmission - Part 1")?;
    let line = input
        .unwrapped_lines()
        .next()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Input has no content"))?;

    let result = line
        .chars()
        .map(|c| c.to_digit(10).expect("Failed to parse FFT input") as i32)
        .collect();

    Ok(result)
}

fn get_pattern(position: usize) -> impl Iterator<Item = i32> {
    let base = [0, 1, 0, -1];

    base.into_iter()
        .flat_map(move |v| itertools::repeat_n(v, position))
        .cycle()
        .skip(1)
}

fn compute_phase(data: &[i32]) -> Vec<i32> {
    (1..data.len() + 1)
        .map(|pos| {
            let pattern = get_pattern(pos);

            let summed: i32 = data.iter().zip(pattern).map(|(v, pat)| v * pat).sum();

            summed.abs() % 10
        })
        .collect()
}

fn run_phases(initial: Vec<i32>, n: usize) -> Vec<i32> {
    iterate(initial, |d| compute_phase(d)).nth(n).unwrap()
}

fn main() -> std::io::Result<()> {
    let received_data = parse_input()?;
    let computed = run_phases(received_data, 100);

    let code: String = computed[..8]
        .iter()
        .map(|digit| std::char::from_digit(*digit as u32, 10).unwrap())
        .collect();

    println!("{}", code);

    Ok(())
}
