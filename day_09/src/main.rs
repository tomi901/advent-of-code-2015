use std::path::Path;
use std::str::FromStr;
use anyhow::{self, Context};
use day_09::PathMap;
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
    let map = PathMap::from_str(input)?;
    // dbg!(&map);

    let result = map.get_shortest_path().context("No path found")?;
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let map = PathMap::from_str(input)?;
    let result = map.get_longest_path().context("No path found")?;
    display_result(&result);
    Ok(())
}
