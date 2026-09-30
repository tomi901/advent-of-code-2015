use std::path::Path;
use std::str::FromStr;
use anyhow::{self, Context};
use day_13::{find_most_optimal_happiness, GuestList};
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
    let guest_list = GuestList::from_str(input)?;
    // dbg!(guest_list);
    let (result, result_list) = find_most_optimal_happiness(&guest_list)
        .context("No result found")?;
    let arranged_list = result_list
        .iter()
        .map(|&i| guest_list.get_name(i).to_string())
        .collect::<Vec<_>>();
    dbg!(arranged_list);
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");

    Ok(())
}
