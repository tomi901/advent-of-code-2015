use std::path::Path;
use std::str::FromStr;
use anyhow::{self, Context};
use day_15::{calculate_best_recipe, calculate_best_recipe_for_calories, Ingredient};
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
    let ingredients = input
        .lines()
        .map(Ingredient::from_str)
        .collect::<Result<Vec<Ingredient>, _>>()?;
    let result = calculate_best_recipe(&ingredients[..], 100)
        .context("No recipe found")?;
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let ingredients = input
        .lines()
        .map(Ingredient::from_str)
        .collect::<Result<Vec<Ingredient>, _>>()?;
    let result = calculate_best_recipe_for_calories(
            &ingredients[..],
            100,
            500,
        )
        .context("No recipe found")?;
    display_result(&result);
    Ok(())
}
