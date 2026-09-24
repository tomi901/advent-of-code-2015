use std::path::Path;
use anyhow::{anyhow, Context};
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

    let result: i64 = input
        .chars()
        .try_fold(0i64, |acc, c| {
            get_movement(c).map(|mov| acc + mov)
        })?;
    
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");

    let mut floor = 0;
    for (index, char) in input.chars().enumerate() {
        floor += get_movement(char)?;

        if floor < 0 {
            display_result(&(index + 1));
            return Ok(());
        }
    }

    Err(anyhow!("Invalid input, didn't reach -1"))
}

fn get_movement(c: char) -> anyhow::Result<i64> {
    match c {
        '(' => Ok(1),
        ')' => Ok(-1),
        _ => Err(anyhow!("Invalid character '{}'", c)),
    }
}
