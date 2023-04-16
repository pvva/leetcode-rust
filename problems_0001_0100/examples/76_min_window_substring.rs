/*
https://leetcode.com/problems/minimum-window-substring/

Given two strings s and t of lengths m and n respectively, return the minimum window substring of
s such that every character in t (including duplicates) is included in the window. If there is no
such substring, return the empty string "".

The testcases will be generated such that the answer is unique.

A substring is a contiguous sequence of characters within the string.

 Example 1:

Input: s = "ADOBECODEBANC", t = "ABC"
Output: "BANC"
Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.

Example 2:

Input: s = "a", t = "a"
Output: "a"
Explanation: The entire string s is the minimum window.

Example 3:

Input: s = "a", t = "aa"
Output: ""
Explanation: Both 'a's from t must be included in the window.
Since the largest window of s only has one 'a', return empty string.

Constraints:

m == s.length
n == t.length
1 <= m, n <= 10^5
s and t consist of uppercase and lowercase English letters.
*/

struct MinWindowSubstringTestCase {
    input_s: String,
    input_t: String,
    output: String,
}

fn new_min_window_substring_test_case(s: &str, t: &str, out: &str) -> MinWindowSubstringTestCase {
    MinWindowSubstringTestCase {
        input_s: s.to_string(),
        input_t: t.to_string(),
        output: out.to_string(),
    }
}

fn main() {
    let cases = vec![
        new_min_window_substring_test_case("ADOBECODEBANC", "ABC", "BANC"),
        new_min_window_substring_test_case("acbbaca", "aba", "baca"),
    ];

    for case in cases {
        println!(
            "for input '{}' and '{}' expected result is '{}', got '{}'",
            case.input_s.clone(),
            case.input_t.clone(),
            case.output.clone(),
            Solution::min_window(case.input_s, case.input_t)
        );
    }
}

struct Solution;

// solution is below

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut mem_map = HashMap::<char, i32>::new();

        for (_, c) in t.chars().enumerate() {
            if let Some(val) = mem_map.get_mut(&c) {
                *val += 1;
            } else {
                mem_map.insert(c, 1);
            }
        }

        let mut diff = t.len();
        let mut min_len = usize::MAX;
        let mut start = 0;
        let mut head = 0;
        let mut idx = 0;
        let cs = s.as_bytes();

        while idx < cs.len() {
            let mut cv = 0;
            let c = cs[idx] as char;
            if let Some(val) = mem_map.get(&c) {
                if *val > 0 {
                    diff -= 1;
                }
                cv = *val;
            }
            mem_map.insert(c, cv - 1);
            idx += 1;

            while diff == 0 {
                if idx - start < min_len {
                    min_len = idx - start;
                    head = start;
                }

                let c0 = cs[start] as char;
                if let Some(val0) = mem_map.get_mut(&c0) {
                    if *val0 == 0 {
                        diff += 1;
                    }
                    *val0 += 1;
                    start += 1;
                }
            }
        }

        if min_len == usize::MAX {
            return "".to_string();
        }

        String::from_utf8(cs[head..head + min_len].to_vec()).unwrap()
    }
}
