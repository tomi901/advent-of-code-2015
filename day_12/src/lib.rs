
pub fn get_nums_sum(s: &str) -> i64 {
    let mut sum = 0;
    let mut current_number = 0;
    let mut is_negative = false;

    for char in s.chars() {
        let Some(digit) = char.to_digit(10) else {
            sum += if is_negative { -current_number } else { current_number };
            current_number = 0;
            is_negative = char == '-';
            continue;
        };

        current_number *= 10;
        current_number += digit as i64;
    }
    sum + if is_negative { -current_number } else { current_number }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part_1_case_1() {
        assert_eq!(get_nums_sum("[1,2,3]"), 6);
    }

    #[test]
    pub fn part_1_case_2() {
        assert_eq!(get_nums_sum(r#"{"a":{"b":4},"c":-1}"#), 3);
    }
}
