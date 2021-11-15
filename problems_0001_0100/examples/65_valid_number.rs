/*
https://leetcode.com/problems/valid-number/

Validate if a given string can be interpreted as a decimal number.

Some examples:
"0" => true
" 0.1 " => true
"abc" => false
"1 a" => false
"2e10" => true
" -90e3   " => true
" 1e" => false
"e3" => false
" 6e-1" => true
" 99e2.5 " => false
"53.5e93" => true
" --6 " => false
"-+3" => false
"95a54e53" => false
*/

struct ValidNumberTestCase {
    input: String,
    is_number: bool,
}

fn new_vn_test_case(s: &str, b: bool) -> ValidNumberTestCase {
    ValidNumberTestCase {
        input: s.to_string(),
        is_number: b,
    }
}

fn main() {
    let cases = vec![
        new_vn_test_case("0", true),
        new_vn_test_case(" 0.1", true),
        new_vn_test_case("abc", false),
        new_vn_test_case("1 a", false),
        new_vn_test_case("2e10", true),
        new_vn_test_case(" -90e3", true),
        new_vn_test_case(" 1e", false),
        new_vn_test_case("e3", false),
        new_vn_test_case(" 6e-1", true),
        new_vn_test_case(" 99e2.5", false),
        new_vn_test_case("53.5e93", true),
        new_vn_test_case(" --6", false),
        new_vn_test_case("-+3", false),
        new_vn_test_case("95a54e53", false),
    ];

    for case in cases {
        println!(
            "{} => {} / {}",
            case.input.clone(),
            Solution::is_number(case.input),
            case.is_number
        );
    }
}

struct Solution;

// solution is below

impl Solution {
    pub fn is_number(s: String) -> bool {
        let parts: Vec<&str> = s.trim().split("e").collect();
        let l = parts.len();

        if l < 1 || l > 2 {
            return false;
        }

        let mut is_n = Self::is_numeric(parts[0].to_string(), false);
        if l == 2 {
            is_n = is_n && Self::is_numeric(parts[1].to_string(), true)
        }

        is_n
    }

    pub fn is_numeric(s: String, is_int: bool) -> bool {
        if s.is_empty() || (is_int && s.contains(".")) {
            return false;
        }

        let mut has_dot = false;
        let mut has_digit = false;
        let mut start = 0;

        let chars: Vec<char> = s.chars().collect();

        if chars[0] == '-' || chars[0] == '+' {
            start = 1;
        }

        for idx in start..s.len() {
            if chars[idx] == '.' {
                if has_dot {
                    return false;
                }
                has_dot = true;

                continue;
            }

            if chars[idx] < '0' || chars[idx] > '9' {
                return false;
            }
            has_digit = true;
        }

        has_digit
    }
}
