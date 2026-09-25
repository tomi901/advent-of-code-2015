use std::path::Path;
use anyhow::{self, Context};
use day_05::{is_nice_string, is_nice_string_updated};
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
    let result = input
        .lines()
        .filter(|&l| is_nice_string(l))
        .count();
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let result = input
        .lines()
        .filter(|&l| is_nice_string_updated(l))
        .count();
    display_result(&result);
    Ok(())
}
