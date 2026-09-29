use json::JsonValue;
use json::object::Object;

pub fn get_json_sum(s: &str) -> anyhow::Result<i64> {
    let data = json::parse(s)?;
    Ok(get_json_sum_from_value(&data, &mut |_| true))
}

pub fn get_json_sum_no_red(s: &str) -> anyhow::Result<i64> {
    let data = json::parse(s)?;
    Ok(get_json_sum_from_value(&data, &mut |v| match v {
        JsonValue::Object(object) => !has_red(object),
        _ => true
    }))
}

fn get_json_sum_from_value(value: &JsonValue, filter: &mut impl FnMut(&JsonValue) -> bool) -> i64 {
    if !filter(value) {
        return 0;
    }

    match value {
        JsonValue::Number(n) => n.as_fixed_point_i64(0).unwrap(),
        JsonValue::Object(object) => object.iter()
            .map(|(_, v)| get_json_sum_from_value(v, filter))
            .sum(),
        JsonValue::Array(array) => array
            .iter()
            .map(|v| get_json_sum_from_value(v, filter))
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
        assert_eq!(get_json_sum("[1,2,3]").unwrap(), 6);
    }

    #[test]
    pub fn part_1_case_2() {
        assert_eq!(get_json_sum(r#"{"a":{"b":4},"c":-1}"#).unwrap(), 3);
    }

    #[test]
    pub fn part_1_case_3() {
        assert_eq!(get_json_sum(r#"[1,{"c":"red","b":2},3]"#).unwrap(), 6);
    }

    #[test]
    pub fn part_2_case_1() {
        assert_eq!(get_json_sum_no_red(r#"[1,{"c":"red","b":2},3]"#).unwrap(), 4);
    }
}
