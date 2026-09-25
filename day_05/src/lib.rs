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

pub fn is_nice_string_updated(s: &str) -> bool {
    has_3_mirror_rule(s) && has_double_pair_rule(s)
}

fn has_double_pair_rule(s: &str) -> bool {
    const SAMPLE_LEN: usize = 2;
    let upper_limit = s.len() - SAMPLE_LEN;
    for i in 0..=upper_limit {
        let sample_limit = i + SAMPLE_LEN;
        let left = &s[i..sample_limit];
        for j in sample_limit..=upper_limit {
            let right = &s[j..j + 2];
            if left == right {
                return true;
            }
        }
    }

    false
}

fn has_3_mirror_rule(s: &str) -> bool {
    (0..=s.len() - 3)
        .map(|i| &s[i..i + 3])
        .any(|x| x.ends_with(x.chars().next().unwrap()))
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

    #[test]
    pub fn double_pair_works_correctly() {
        assert!(has_double_pair_rule("xyxy"));
        assert!(has_double_pair_rule("aabcdefgaa"));
        assert!(!has_double_pair_rule("aaa"));
    }

    #[test]
    pub fn three_mirror_works_correctly() {
        assert!(has_3_mirror_rule("xyx"));
        assert!(has_3_mirror_rule("abcdefeghi"));
        assert!(has_3_mirror_rule("aaa"));
        assert!(!has_3_mirror_rule("aabcdefgaa"));
    }

    #[test]
    pub fn part_2_case_1() {
        assert!(is_nice_string_updated("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    pub fn part_2_case_2() {
        assert!(is_nice_string_updated("xxyxx"));
    }

    #[test]
    pub fn part_2_case_3() {
        assert!(!is_nice_string_updated("uurcxstgmygtbstg"));
    }

    #[test]
    pub fn part_2_case_4() {
        assert!(!is_nice_string_updated("ieodomkazucvgmuy"));
    }
}
