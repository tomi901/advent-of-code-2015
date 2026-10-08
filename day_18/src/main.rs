use std::path::Path;
use std::str::FromStr;
use anyhow::{self, Context};
use day_18::{alive_count, simulate_step};
use xmas::display_result;
use xmas::map2d::ByteMap;

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
    let mut state = ByteMap::from_str(input)?;
    for _ in 0..100 {
        state = simulate_step(&state);
    }

    let result = alive_count(state);
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");

    Ok(())
}
