use std::collections::HashSet;
use std::path::Path;
use anyhow::{self, Context};
use xmas::direction::Direction;
use xmas::display_result;
use xmas::point2d::Point2D;

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
    let directions = input
        .chars()
        .map(|c| Direction::from_char(c).with_context(|| format!("Invalid direction: {c}")));

    // Naive implementation, probably a Set will occupy too much in memory
    let mut cur = Point2D::ZERO;
    let mut visited = HashSet::new();
    visited.insert(cur);

    for dir in directions {
        let dir = dir?;
        cur += dir.as_point();
        visited.insert(cur);
    }

    let result = visited.len();
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let directions = input
        .chars()
        .map(|c| Direction::from_char(c).with_context(|| format!("Invalid direction: {c}")));

    let mut santa = Point2D::ZERO;
    let mut robo_santa = Point2D::ZERO;
    let mut visited = HashSet::new();
    visited.insert(santa);

    for (i, dir) in directions.enumerate() {
        let dir = dir?;
        if i % 2 == 0 {
            santa += dir.as_point();
            visited.insert(santa);
        } else {
            robo_santa += dir.as_point();
            visited.insert(robo_santa);
        }
    }

    let result = visited.len();
    display_result(&result);
    Ok(())
}
