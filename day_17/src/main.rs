use std::path::Path;
use std::str::FromStr;
use anyhow::{self, Context};
use day_17::{calculate_combinations, calculate_combinations_minimum_containers};
use xmas::display_result;

fn main() -> anyhow::Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("input.txt");
    let input = std::fs::read_to_string(path)
        .context("Error reading input file.")?;

    part_1(&input)?;
    println!();
    part_2(&input)?;
    Ok(())
}

fn part_1(input: &str) -> anyhow::Result<()> {
    println!("Part 1:");
    let containers = input
        .lines()
        .map(u32::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    let result = calculate_combinations(150, &containers);
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let containers = input
        .lines()
        .map(u32::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    let result = calculate_combinations_minimum_containers(150, &containers);
    display_result(&result);
    Ok(())
}
