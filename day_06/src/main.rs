use std::path::Path;
use std::str::FromStr;
use anyhow::{self, Context};
use day_06::{Instruction, InstructionKind, Lights};
use xmas::display_result;

fn main() -> anyhow::Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("input.txt");
    let input = std::fs::read_to_string(path)
        .context("Error reading input file.")?;

    // part_1(&input)?;
    println!();
    part_2(&input)?;
    Ok(())
}

fn part_1(input: &str) -> anyhow::Result<()> {
    println!("Part 1:");
    let mut lights = Lights::new();
    for line in input.lines() {
        lights.apply_instruction(line)?;
    }
    let result = lights.lit_count();
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let result = input
        .lines()
        .try_fold(0, |acc, line| -> anyhow::Result<_> {
            let instruction = Instruction::from_str(line)?;
            let delta = instruction.brightness_delta();
            Ok(acc + delta)
        })?;
    display_result(&result);
    Ok(())
}
