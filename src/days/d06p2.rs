use aoc_utils::BufferedInput;
use itertools::{EitherOrBoth, Itertools};
use std::collections::HashMap;

type OrbitMap = HashMap<String, Vec<String>>;

fn parse_input() -> std::io::Result<OrbitMap> {
    let input = BufferedInput::parse_args("Day 6: Universal Orbit Map - Part 2")?;
    let lines = input.unwrapped_lines();

    let result = lines
        .map(|line| line.split(')').map_into().collect_tuple().unwrap())
        .into_group_map();

    Ok(result)
}

fn find_path_to<'a>(orbits: &'a OrbitMap, target: &str) -> Option<Vec<&'a str>> {
    let mut searchspace = vec![("COM", vec![])];

    while let Some((center, path)) = searchspace.pop() {
        if center == target {
            return Some(path);
        }

        if let Some(neighbors) = orbits.get(center) {
            let new_searches = neighbors.iter().map(|body| {
                let mut new_path = path.clone();
                new_path.push(center);
                (body.as_str(), new_path)
            });
            searchspace.extend(new_searches);
        }
    }

    None
}

fn main() -> std::io::Result<()> {
    let orbits = parse_input()?;

    let self_path = find_path_to(&orbits, "YOU").expect("Path to self not found");
    let santa_path = find_path_to(&orbits, "SAN").expect("Path to Santa not found");

    let zipped = self_path.into_iter().zip_longest(santa_path);

    let split_paths = zipped.skip_while(|either| {
        let both = either.as_ref().both();
        itertools::any(both, |(a, b)| a == b)
    });

    let transfers: i32 = split_paths
        .map(|either| match either {
            EitherOrBoth::Both(_, _) => 2,
            _ => 1,
        })
        .sum();

    println!("{}", transfers);

    Ok(())
}
