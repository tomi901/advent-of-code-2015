
pub fn next_valid_password(s: &str) -> String {
    let mut result = s.to_string();
    loop {
        unsafe {
            let bytes = result.as_mut_vec();
            next_password(&mut bytes[..]);
        };

        if is_valid_password(&result) {
            return result;
        }
    }
}

unsafe fn next_password(s: &mut [u8]) {
    let mut carry = true;
    for i in (0..s.len()).rev() {
        if !carry {
            break
        }

        match s[i] {
            b'a'..=b'y' => {
                carry = false;
                s[i] += 1;
            },
            b'z' => {
                carry = true;
                s[i] = b'a';
            }
            _ => unreachable!("Invalid character: {}", s[i]),
        }
    }
}

fn is_valid_password(s: &str) -> bool {
    has_three_straight(s) && !has_commonly_mistaken_letters(s) && has_double_pair(s)
}

fn has_three_straight(s: &str) -> bool {
    let Some(mut previous) = s.bytes().next() else {
        return false;
    };

    let mut straight_count = 1;
    for current in s.bytes().skip(1) {
        if (previous + 1) != current {
            previous = current;
            straight_count = 1;
            continue;
        }

        previous = current;
        straight_count += 1;
        if straight_count == 3 {
            return true;
        }
    }
    false
}

fn has_commonly_mistaken_letters(s: &str) -> bool {
    s.chars()
        .any(|c| match c {
            'i' | 'o' | 'l' => true,
            _ => false
        })
}

fn has_double_pair(s: &str) -> bool {
    let Some(mut previous) = s.chars().next() else {
        return false;
    };

    let mut pair_count = 0;
    let mut skip = false;
    for current in s.chars().skip(1) {
        if !skip && current == previous {
            pair_count += 1;
            if pair_count >= 2 {
                return true;
            }
            skip = true;
            continue;
        }
        
        previous = current;
        skip = false;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn has_three_straight_positive_case() {
        assert!(has_three_straight("xyz"));
    }

    #[test]
    pub fn has_three_straight_positive_long_case() {
        assert!(has_three_straight("testhijklmmn"));
    }

    #[test]
    pub fn has_three_straight_negative_case() {
        assert!(!has_three_straight("abd"));
    }

    #[test]
    pub fn is_valid_password_case_1() {
        assert!(!is_valid_password("hijklmmn"));
    }

    #[test]
    pub fn is_valid_password_case_2() {
        assert!(!is_valid_password("abbceffg"));
    }

    #[test]
    pub fn is_valid_password_case_3() {
        assert!(!is_valid_password("abbcegjk"));
    }

    #[test]
    pub fn is_valid_password_case_4() {
        assert!(is_valid_password("abcdffaa"));
    }

    #[test]
    pub fn is_valid_password_case_5() {
        assert!(is_valid_password("ghjaabcc"));
    }

    #[test]
    pub fn part_1_case_1() {
        assert!(next_valid_password("abcdefgh") <= "abcdffaa".to_string());
        assert_eq!(next_valid_password("abcdefgh"), "abcdffaa");
    }

    #[test]
    pub fn part_1_case_2() {
        assert!(next_valid_password("ghijklmn") <= "ghjaabcc".to_string());
        assert_eq!(next_valid_password("ghijklmn"), "ghjaabcc");
    }
}
