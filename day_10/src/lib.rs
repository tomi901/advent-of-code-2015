use std::fmt::Write;
use anyhow::Context;

pub fn look_and_say(s: &str) -> anyhow::Result<String> {
    let mut last_digit = s
        .chars()
        .next()
        .context("Empty string")?
        .to_digit(10)
        .context("Not a digit")?;
    let mut digits_count = 1;
    let mut result = String::new();

    for c in s.chars().skip(1) {
        let digit = c.to_digit(10).context("Not a digit")?;
        if digit != last_digit {
            write!(result, "{}{}", digits_count, last_digit)?;
            last_digit = digit;
            digits_count = 0;
        }
        digits_count += 1;
    }
    write!(result, "{}{}", digits_count, last_digit)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part_1_case_1() {
        assert_eq!(look_and_say("1").unwrap(), "11");
    }

    #[test]
    pub fn part_1_case_2() {
        assert_eq!(look_and_say("111221").unwrap(), "312211");
    }
}
