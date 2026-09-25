use std::path::Path;
use anyhow::{self, Context};
use day_07::{Circuit, CircuitCache, Expression};
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
    let circuit = input.parse::<Circuit>()?;
    let mut cache = CircuitCache::default();
    let result = circuit.evaluate("a", &mut cache)?;
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let mut circuit = input.parse::<Circuit>()?;
    let mut cache = CircuitCache::default();
    let previous_result = circuit.evaluate("a", &mut cache)?;
    
    cache.clear();
    circuit.set("b".to_string(), Expression::constant(previous_result));
    let result = circuit.evaluate("a", &mut cache)?;
    
    display_result(&result);
    Ok(())
}
