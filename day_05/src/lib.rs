pub fn is_nice_string(s: &str) -> bool {
    const FORBIDDEN: [&str; 4] = [
        "ab",
        "cd",
        "pq",
        "xy",
    ];
    if FORBIDDEN.iter().any(|x| s.contains(x)) {
        return false;
    }

    let mut vowel_count = 0;
    let mut previous_char = char::MIN;
    let mut has_double_letter = false;
    for c in s.chars() {
        if c == previous_char {
            has_double_letter = true;
        }

        if "aeiou".contains(c) {
            vowel_count += 1;
        }
        previous_char = c;
    }

    has_double_letter && vowel_count >= 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part_1_case_1() {
        let is_nice = is_nice_string("ugknbfddgicrmopn");
        assert!(is_nice);
    }

    #[test]
    pub fn part_1_case_2() {
        let is_nice = is_nice_string("aaa");
        assert!(is_nice);
    }

    #[test]
    pub fn part_1_case_3() {
        let is_nice = is_nice_string("jchzalrnumimnmhp");
        assert!(!is_nice);
    }

    #[test]
    pub fn part_1_case_4() {
        let is_nice = is_nice_string("haegwjzuvuyypxyu");
        assert!(!is_nice);
    }

    #[test]
    pub fn part_1_case_5() {
        let is_nice = is_nice_string("dvszwmarrgswjxmb");
        assert!(!is_nice);
    }
}
