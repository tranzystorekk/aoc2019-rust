use aoc_utils::BufferedInput;
use itertools::Itertools;
use std::collections::HashMap;

type OrbitMap = HashMap<String, Vec<String>>;

fn parse_input() -> std::io::Result<OrbitMap> {
    let input = BufferedInput::parse_args("Day 6: Universal Orbit Map - Part 1")?;
    let lines = input.unwrapped_lines();

    let result = lines
        .map(|line| line.split(')').map_into().collect_tuple().unwrap())
        .into_group_map();

    Ok(result)
}

fn calculate_orbit_checksum(orbits: OrbitMap) -> i32 {
    let mut searchspace: Vec<(String, _)> = vec![("COM".into(), 0)];
    let mut checksum = 0;

    while let Some((center, length)) = searchspace.pop() {
        if let Some(neighbors) = orbits.get(&center) {
            let new_searches = neighbors.iter().cloned().map(|body| (body, length + 1));
            searchspace.extend(new_searches);
        }

        checksum += length;
    }

    checksum
}

fn main() -> std::io::Result<()> {
    let orbits = parse_input()?;

    let checksum = calculate_orbit_checksum(orbits);

    println!("{}", checksum);

    Ok(())
}
