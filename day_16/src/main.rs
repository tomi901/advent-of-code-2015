use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;
use anyhow::{self, Context};
use day_16::Sue;
use xmas::display_result;
use lazy_static::lazy_static;

lazy_static! {
    static ref HINTS: HashMap<String, u32> = {
        let mut hints = HashMap::new();
        hints.insert("children".to_string(), 3);
        hints.insert("cats".to_string(), 7);
        hints.insert("samoyeds".to_string(), 2);
        hints.insert("pomeranians".to_string(), 3);
        hints.insert("akitas".to_string(), 0);
        hints.insert("vizslas".to_string(), 0);
        hints.insert("goldfish".to_string(), 5);
        hints.insert("trees".to_string(), 3);
        hints.insert("cars".to_string(), 2);
        hints.insert("perfumes".to_string(), 1);
        hints
    };
}

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
    let aunts = input
        .lines()
        .map(Sue::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    let result = aunts
        .iter()
        .filter(|&s| s.is_suspect(&HINTS))
        .next()
        .context("None found")?
        .number();

    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let aunts = input
        .lines()
        .map(Sue::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let result = aunts
        .iter()
        .filter(|&s| s.is_suspect_v2(&HINTS))
        .next()
        .context("None found")?
        .number();
    
    display_result(&result);
    Ok(())
}
