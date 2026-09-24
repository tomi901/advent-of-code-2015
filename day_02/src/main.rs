use std::path::Path;
use anyhow::{self, Context};
use day_02::GiftBox;
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
    
    let mut result = 0;
    for line in input.lines() {
        let gift = line.parse::<GiftBox>()?;
        result += gift.surface_area_needed();
    }

    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");

    Ok(())
}
