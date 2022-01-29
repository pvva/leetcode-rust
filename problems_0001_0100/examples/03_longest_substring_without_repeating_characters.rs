/*
https://leetcode.com/problems/longest-substring-without-repeating-characters/

Given a string s, find the length of the longest substring without repeating characters.

 Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.

Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

Example 4:

Input: s = ""
Output: 0

Constraints:

0 <= s.length <= 5 * 104
s consists of English letters, digits, symbols and spaces.
 */

struct LongestSubstringWithoutRepeatingCharactersTestCase {
    input: String,
    output: i32,
}

fn main() {
    let cases = vec![
        LongestSubstringWithoutRepeatingCharactersTestCase {
            input: "abcabcbb".to_string(),
            output: 3,
        },
        LongestSubstringWithoutRepeatingCharactersTestCase {
            input: "bbbbb".to_string(),
            output: 1,
        },
        LongestSubstringWithoutRepeatingCharactersTestCase {
            input: "pwwkew".to_string(),
            output: 3,
        },
        LongestSubstringWithoutRepeatingCharactersTestCase {
            input: "".to_string(),
            output: 0,
        },
    ];

    for case in cases {
        println!(
            "for string '{}' expected result is {}, got {}",
            case.input.clone(),
            case.output,
            Solution::length_of_longest_substring(case.input)
        )
    }
}

struct Solution;

// Solution goes below

use std::cmp::max;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();

        let chars = s.as_bytes();
        let mut max_substr_len: i32 = 0;
        let mut s_idx: usize = 0;
        for (i, c) in chars.iter().enumerate() {
            if set.contains(c) {
                while s_idx < i {
                    let oc = chars[s_idx];
                    set.remove(&oc);
                    s_idx += 1;
                    if oc == *c {
                        break;
                    }
                }
            }
            set.insert(c);
            max_substr_len = max(set.len() as i32, max_substr_len);
        }

        max_substr_len
    }
}
