use std::io::{BufRead, Error, ErrorKind};
use aoc::utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<i32>> {
    let input = BufferedInput::parse_args("Day 16: Flawed Frequency Transmission - Part 1")?;
    let line = input.lines()
        .map(Result::unwrap)
        .next()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Input has no content"))?;

    let result = line.as_str()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    Ok(result)
}

fn get_pattern(position: usize) -> impl Iterator<Item = i32> {
    let base = vec![0, 1, 0, -1];

    base.into_iter()
        .flat_map(move |v| std::iter::repeat(v).take(position))
        .cycle()
        .skip(1)
}

fn compute_phase(data: Vec<i32>) -> Vec<i32> {
    (1..).take(data.len())
        .map(|pos| {
        let pattern = get_pattern(pos);

        let summed: i32 = data.iter().zip(pattern).map(|(v, pat)| v * pat).sum();

        summed.abs() % 10
    })
        .collect()
}

fn run_phases(initial: Vec<i32>, n: usize) -> Vec<i32> {
    std::iter::successors(Some(initial), |data| Some(compute_phase(data.clone())))
        .take(n+1)
        .last()
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let received_data = parse_input()?;
    let computed = run_phases(received_data, 100);

    let code: String = computed[..8].iter()
        .map(|digit| std::char::from_digit(*digit as u32, 10).unwrap())
        .collect();

    println!("{}", code);

    Ok(())
}
