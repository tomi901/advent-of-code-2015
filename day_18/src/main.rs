use std::path::Path;
use std::str::FromStr;
use anyhow::{self, Context};
use day_18::{alive_count, simulate_step, simulate_step_corners_on, ON};
use xmas::display_result;
use xmas::map2d::ByteMap;
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
    let mut state = ByteMap::from_str(input)?;
    for _ in 0..100 {
        state = simulate_step(&state);
    }

    // println!("{}", state);

    let result = alive_count(&state);
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let mut state = ByteMap::from_str(input)?;
    let max_x = state.width() as isize - 1;
    let max_y = state.height() as isize - 1;
    state.set_tile(Point2D(0, 0), ON);
    state.set_tile(Point2D(0, max_y), ON);
    state.set_tile(Point2D(max_x, 0), ON);
    state.set_tile(Point2D(max_x, max_y), ON);


    for _ in 0..100 {
        state = simulate_step_corners_on(&state);
    }

    // println!("{}", state);

    let result = alive_count(&state);
    display_result(&result);
    Ok(())
}
