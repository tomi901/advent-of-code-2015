use std::path::Path;
use anyhow::{self, Context};
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
    let raw_len: usize = input
        .lines()
        .map(|l| l.len())
        .sum();
    let new_len: usize = input
        .lines()
        .map(get_unescaped_len)
        .sum();

    let result = raw_len - new_len;
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let raw_len: usize = input
        .lines()
        .map(|l| l.len())
        .sum();
    let new_len: usize = input
        .lines()
        .map(get_escaped_len)
        .sum();

    let result = new_len - raw_len;
    display_result(&result);
    Ok(())
}

fn get_unescaped_len(s: &str) -> usize {
    let trimmed = s.trim().trim_start_matches('"').trim_end_matches('"');

    let mut escaped_len = 0;
    let mut i = 0;
    while i < trimmed.len() {
        escaped_len += 1;

        let cur = &trimmed[i..=i];
        let is_escape = cur == "\\";
        if is_escape && (i + 1) < trimmed.len() && &trimmed[i + 1..=i + 1] == "x" {
            i += 4;
        } else if is_escape {
            i += 2;
        } else {
            i += 1
        }
    }
    escaped_len
}

fn get_escaped_len(s: &str) -> usize {
    let new_len: usize = s
        .chars()
        .map(|c| match c {
            '\\' | '"' => 2,
            _ => 1,
        })
        .sum();
    new_len + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part_1_case_1() {
        let s = r#""""#;
        let new_len = get_unescaped_len(s);
        assert_eq!(s.len(), 2);
        assert_eq!(new_len, 0);
    }

    #[test]
    pub fn part_1_case_2() {
        let s = r#""abc""#;
        let new_len = get_unescaped_len(s);
        assert_eq!(s.len(), 5);
        assert_eq!(new_len, 3);
    }

    #[test]
    pub fn part_2_case_1() {
        let s = r#""""#;
        let new_len = get_escaped_len(s);
        assert_eq!(s.len(), 2);
        assert_eq!(new_len, 6);
    }

    #[test]
    pub fn part_2_case_2() {
        let s = r#""abc""#;
        let new_len = get_escaped_len(s);
        assert_eq!(s.len(), 5);
        assert_eq!(new_len, 9);
    }

    #[test]
    pub fn part_2_case_3() {
        let s = r#""aaa\"aaa""#;
        let new_len = get_escaped_len(s);
        assert_eq!(s.len(), 10);
        assert_eq!(new_len, 16);
    }
}
