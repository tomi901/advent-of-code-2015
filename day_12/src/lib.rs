use json::JsonValue;
use json::object::Object;

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

pub fn get_json_sum(s: &str) -> anyhow::Result<i64> {
    let data = json::parse(s)?;
    Ok(get_json_sum_from_value(&data))
}

fn get_json_sum_from_value(value: &JsonValue) -> i64 {
    match value {
        JsonValue::Number(n) => n.as_fixed_point_i64(0).unwrap(),
        JsonValue::Object(object) => if !has_red(object) {
            object.iter()
                .map(|(_, v)| get_json_sum_from_value(v))
                .sum()
        } else {
            0
        },
        JsonValue::Array(array) => array
            .iter()
            .map(get_json_sum_from_value)
            .sum(),
        _ => 0,
    }
}

fn has_red(object: &Object) -> bool {
    object.iter()
        .any(|(_, v)| is_red_value(v))
}

fn is_red_value(value: &JsonValue) -> bool {
    match value {
        JsonValue::Short(s) => s == "red",
        JsonValue::String(s) => s == "red",
        _ => false
    }
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

    #[test]
    pub fn part_2_case_1() {
        assert_eq!(get_json_sum(r#"[1,{"c":"red","b":2},3]"#).unwrap(), 4);
    }
}
