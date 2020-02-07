/*
https://leetcode.com/problems/strong-password-checker

A password is considered strong if below conditions are all met:

It has at least 6 characters and at most 20 characters.
It must contain at least one lowercase letter, at least one uppercase letter, and at least one digit.
It must NOT contain three repeating characters in a row ("...aaa..." is weak, but "...aa...a..." is strong,
assuming other conditions are met).
Write a function strongPasswordChecker(s), that takes a string s as input, and return the MINIMUM change required
to make s a strong password. If s is already strong, return 0.

Insertion, deletion or replace of any one character are all considered as one change.
*/

struct StrongPasswordCheckerTestCase {
    password: String,
    expected_distance: i32,
}

fn new_sp_test_case(s: &str, d: i32) -> StrongPasswordCheckerTestCase {
    StrongPasswordCheckerTestCase {
        password: s.to_string(),
        expected_distance: d,
    }
}

fn main() {
    let cases = vec![
        new_sp_test_case("5", 5),
        new_sp_test_case("aa", 4),
        new_sp_test_case("aaa", 3),
        new_sp_test_case("123456789aaa123456789", 2),
        new_sp_test_case("aaaaaaaAAAAAA6666bbbbaaaaaaABBC", 13),
    ];

    for case in cases {
        println!(
            "Password: {}, expected distance: {}, calculated distance: {}",
            case.password.clone(),
            case.expected_distance,
            Solution::strong_password_checker(case.password)
        );
    }
}

struct Solution;

// solution goes below

impl Solution {
    pub fn strong_password_checker(s: String) -> i32 {
        let mut distance: i32 = 0;

        let chars: Vec<char> = s.chars().collect();
        let l = chars.len();
        let mut lc_corr = 1;
        let mut uc_corr = 1;
        let mut dc_corr = 1;

        let mut total_replacements_count = 0;
        let mut replacements: Vec<i32> = vec![];
        let mut current_letter: char = '\0';
        let mut current_continuous_fragment_len = 0;

        for i in 0..l {
            if chars[i].is_uppercase() {
                uc_corr = 0;
            }
            if chars[i].is_lowercase() {
                lc_corr = 0;
            }
            if chars[i].is_digit(10) {
                dc_corr = 0;
            }

            if chars[i] != current_letter {
                if current_continuous_fragment_len > 2 {
                    total_replacements_count += current_continuous_fragment_len / 3;
                    replacements.push(current_continuous_fragment_len);
                }
                current_letter = chars[i];
                current_continuous_fragment_len = 1;
            } else {
                current_continuous_fragment_len += 1;
            }
        }
        if current_continuous_fragment_len > 2 {
            total_replacements_count += current_continuous_fragment_len / 3;
            replacements.push(current_continuous_fragment_len);
        }
        let mut corrections = 0;

        if l > 20 {
            distance = l as i32 - 20;
            if total_replacements_count > distance {
                corrections = total_replacements_count - distance;
            } else {
                if replacements.is_empty() {
                    corrections =
                        std::cmp::max(lc_corr + uc_corr + dc_corr, total_replacements_count);
                } else {
                    corrections = std::cmp::max(
                        lc_corr + uc_corr + dc_corr,
                        Self::minimize_replacements_and_removals(replacements, distance as usize),
                    )
                }
            }
        } else if l < 6 {
            distance = 6 - l as i32;
            let additions = distance - (lc_corr + uc_corr + dc_corr);
            if additions < 0 {
                corrections -= additions;
            }
        } else {
            corrections = std::cmp::max(lc_corr + uc_corr + dc_corr, total_replacements_count);
        }

        distance + corrections
    }

    fn minimize_replacements_and_removals(replacements: Vec<i32>, removals: usize) -> i32 {
        let l = replacements.len();

        let mut dynamic_matrix: Vec<Vec<i32>> = vec![];
        for i in 0..removals + 1 {
            dynamic_matrix.push(vec![]);
            dynamic_matrix[i].push(0);
            for _ in 1..l + 1 {
                dynamic_matrix[i].push(std::i32::MAX);
            }
        }

        for j in 1..l + 1 {
            dynamic_matrix[0][j] = dynamic_matrix[0][j - 1] + replacements[j - 1] / 3;
        }
        dynamic_matrix[0][0] = 0;

        for r in 1..removals + 1 {
            for p in 1..l + 1 {
                let mut remove_limit = replacements[p - 1] as usize - 2;
                if r < remove_limit {
                    remove_limit = r;
                }

                for k in 0..remove_limit + 1 {
                    dynamic_matrix[r][p] = std::cmp::min(
                        dynamic_matrix[r][p],
                        dynamic_matrix[k][p - 1] + (replacements[p - 1] - k as i32) / 3,
                    );
                }
            }
        }

        if dynamic_matrix[removals][l] != std::i32::MAX {
            return dynamic_matrix[removals][l];
        }

        0
    }
}
