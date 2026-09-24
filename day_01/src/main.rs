use anyhow::{anyhow, Context};
use xmas::display_result;

fn main() -> anyhow::Result<()> {
    part_1()?;
    println!();
    part_2()?;
    Ok(())
}

fn part_1() -> anyhow::Result<()> {
    println!("Part 1:");
    let input = std::fs::read_to_string("./input.txt")
        .context("Error reading input file.")?;

    let result: i64 = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Invalid character '{}'", c),
        })
        .sum();
    
    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let mut position = 0;
    let mut floor = 0;
    for char in input.chars() {
        position += 1;
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Invalid character '{}'", char),
        }

        if floor < 0 {
            display_result(&position);
            return Ok(());
        }
    }

    Err(anyhow!("Invalid input, didn't reach -1"))
}
