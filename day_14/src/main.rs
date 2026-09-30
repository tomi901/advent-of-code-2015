use std::path::Path;
use std::str::FromStr;
use anyhow::{self, Context};
use day_14::Reindeer;
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
    let reindeers: Vec<_> = input
        .lines()
        .map(Reindeer::from_str)
        .collect::<Result<_, _>>()?;
    let (winner, result) = reindeers
        .iter()
        .map(|r| (r, r.distance_after(2503)))
        .max_by_key(|x| x.1)
        .context("None found")?;
    println!("Winner: {}!", winner.name());
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let reindeers: Vec<_> = input
        .lines()
        .map(Reindeer::from_str)
        .collect::<Result<_, _>>()?;
    let mut score = vec![0; reindeers.len()];

    for seconds in 1..=2503 {
        let lead_distance = reindeers
            .iter()
            .map(|r| r.distance_after(seconds))
            .max()
            .unwrap();

        let lead_reindeers = reindeers
            .iter()
            .enumerate()
            .filter(|(_, r)| r.distance_after(seconds) == lead_distance);
        for (i, _) in lead_reindeers {
            score[i] += 1;
        }
    }

    let result = score.iter().max().unwrap();
    display_result(&result);
    Ok(())
}
