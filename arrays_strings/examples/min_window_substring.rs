/*
Given a string S and a string T, find the minimum window in S which will contain all the characters
in T in complexity O(n).

Example:

Input: S = "ADOBECODEBANC", T = "ABC"
Output: "BANC"
*/

fn main() {
    println!(
        "{:?}",
        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
    );
    println!(
        "{:?}",
        Solution::min_window("acbbaca".to_string(), "aba".to_string())
    );
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
        let mut min_len = std::usize::MAX;
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

        if min_len == std::usize::MAX {
            return "".to_string();
        }

        String::from_utf8(cs[head..head + min_len].to_vec()).unwrap()
    }
}
