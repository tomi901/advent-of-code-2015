use std::path::Path;
use anyhow::{self, Context};
use md5::Digest;
use xmas::display_result;

fn main() -> anyhow::Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("input.txt");
    let input = std::fs::read_to_string(path)
        .context("Error reading input file.")?;

    part_1(&input.trim())?;
    println!();
    part_2(&input.trim())?;
    Ok(())
}

fn part_1(input: &str) -> anyhow::Result<()> {
    println!("Part 1:");
    let result = find_answer(input, 5).context("none found")?;
    display_result(&result);
    Ok(())
}

fn part_2(input: &str) -> anyhow::Result<()> {
    println!("Part 2:");
    let result = find_answer(input, 6).context("none found")?;
    display_result(&result);
    Ok(())
}

fn find_answer(key: &str, leading_zeroes: usize) -> Option<u32> {
    for i in 1..u32::MAX {
        let hash_str = digest_str(key, i);
        if hash_str[..leading_zeroes].chars().all(|c| c == '0') {
            return Some(i);
        }
    }

    None
}

fn digest(key: &str, input: u32) -> Digest {
    let hash_input = format!("{key}{input}");
    md5::compute(hash_input.as_bytes())
}

fn digest_str(key: &str, input: u32) -> String {
    let digest = digest(key, input);
    format!("{:?}", digest)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn hashes_correctly() {
        let digest = digest_str("abcdef", 609043);
        assert!(digest.starts_with("000001dbbfa"), "Incorrect digest: {digest}");
    }

    #[test]
    pub fn part_1_case_1() {
        let answer = find_answer("abcdef", 5);
        assert_eq!(answer, Some(609043));
    }

    #[test]
    pub fn part_1_case_2() {
        let answer = find_answer("pqrstuv", 5);
        assert_eq!(answer, Some(1048970));
    }
}
