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
    let result = input
        .lines()
        .try_fold(0u64, |acc, line| -> anyhow::Result<_> {
            let gift = line.parse::<GiftBox>()?;
            Ok(acc + gift.needed_wrap_surface_area())
        })?;

    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let result = input
        .lines()
        .try_fold(0u64, |acc, line| -> anyhow::Result<_> {
            let gift = line.parse::<GiftBox>()?;
            Ok(acc + gift.needed_ribbon_length())
        })?;

    display_result(&result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    pub fn part_2_case_1() {
        let gift = GiftBox::from_str("2x3x4").unwrap();
        let length = gift.needed_ribbon_length();
        assert_eq!(length, 34);
    }

    #[test]
    pub fn part_2_case_2() {
        let gift = GiftBox::from_str("1x1x10").unwrap();
        let length = gift.needed_ribbon_length();
        assert_eq!(length, 14);
    }
}
