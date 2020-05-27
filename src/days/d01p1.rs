use aoc::utils::BufferedInput;

fn parse_input() -> std::io::Result<Vec<i32>> {
    let input = BufferedInput::parse_args("Day 1: The Tyranny of the Rocket Equation - Part 1")?;
    let lines = input.unwrapped_lines();

    let parsed = lines
        .map(|line| line.parse().expect("Failed to parse module weights"))
        .collect();

    Ok(parsed)
}

fn main() -> std::io::Result<()> {
    let module_weights = parse_input()?;
    let summed_req: i32 = module_weights.into_iter().map(|w| w / 3 - 2).sum();

    println!("{}", summed_req);

    Ok(())
}
