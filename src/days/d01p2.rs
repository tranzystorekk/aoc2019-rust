use std::io::BufRead;
use aoc::utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<i32>> {
    let input = BufferedInput::parse_args("Day 1: The Tyranny of the Rocket Equation - Part 2")?;
    let lines = input.lines().map(Result::unwrap);

    let parsed: Vec<i32> = lines.map(|line| line.parse().expect("Failed to parse module weights")).collect();
    Ok(parsed)
}

fn fuel_req(weight: i32) -> i32 {
    weight / 3 - 2
}

fn fuel_req_chained(weight: i32) -> i32 {
    let start = Some(fuel_req(weight)).filter(|fuel| *fuel > 0);
    let chain = std::iter::successors(start, |val| Some(fuel_req(*val)));

    chain.take_while(|req| *req > 0).sum()
}

fn main() -> std::io::Result<()> {
    let module_weights = parse_input()?;
    let summed_req: i32 = module_weights.into_iter()
        .map(fuel_req_chained)
        .sum();

    println!("{}", summed_req);

    Ok(())
}
