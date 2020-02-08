/*
https://leetcode.com/problems/wildcard-matching/

Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*'.

'?' Matches any single character.
'*' Matches any sequence of characters (including the empty sequence).
The matching should cover the entire input string (not partial).

Note:

s could be empty and contains only lowercase letters a-z.
p could be empty and contains only lowercase letters a-z, and characters like ? or *.

Input:
s = "aa"
p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".

Input:
s = "aa"
p = "*"
Output: true
Explanation: '*' matches any sequence.

Input:
s = "cb"
p = "?a"
Output: false
Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.

Input:
s = "adceb"
p = "*a*b"
Output: true
Explanation: The first '*' matches the empty sequence, while the second '*' matches the substring "dce".
*/

struct WildcardTestCase {
    input: String,
    pattern: String,
    is_match: bool,
}

fn new_w_tc(input: &str, pattern: &str, is_match: bool) -> WildcardTestCase {
    WildcardTestCase {
        input: input.to_string(),
        pattern: pattern.to_string(),
        is_match,
    }
}

fn main() {
    let test_cases = vec![
        //        new_w_tc("aa", "a", false),
        //        new_w_tc("aa", "*", true),
        //        new_w_tc("cb", "?a", false),
        //        new_w_tc("adceb", "*a*b", true),
        //        new_w_tc("acdcb", "a*c?b", false),
        //        new_w_tc("acdcb", "a*c*b", true),
        //        new_w_tc("a", "a", true),
        //        new_w_tc("a", "?", true),
        //        new_w_tc("a", "?a", false),
        //        new_w_tc("aa", "?a", true),
        //        new_w_tc("abefcdgiescdfimde", "ab*cd?i*de", true),
        //        new_w_tc("aaaa", "***a", true),
        //        new_w_tc("aaaa", "***aa", true),
        //        new_w_tc("aaaa", "***aaa", true),
        //        new_w_tc("", "*", true),
        //        new_w_tc("", "**", true),
        //        new_w_tc("bcd", "??", false),
        //        new_w_tc("bcd", "*?*", true),
        //        new_w_tc("abc", "*ab", false),
        //        new_w_tc("abc", "*bc", true),
        //        new_w_tc("hi", "*?", true),
        //        new_w_tc("hi", "*??", true),
        //        new_w_tc("b", "?", true),
        //        new_w_tc("b", "?*?", false),
        //        new_w_tc("mississippi", "m*iss*iss*", true),
        //        new_w_tc("mississippi", "m*si*", true),
        new_w_tc("abcde", "*?*?*?*?", true),
    ];

    for case in test_cases {
        println!(
            "match is {}, expected is {} for word {} and pattern {}",
            Solution::is_match(case.input.clone(), case.pattern.clone()),
            case.is_match,
            case.input,
            case.pattern
        )
    }
}

struct Solution;

// solution goes below

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let input_chars: Vec<char> = s.chars().collect();
        let pattern_chars: Vec<char> = p.chars().collect();

        let mut input_idx = 0;
        let mut pattern_idx = 0;
        let mut current_match_idx = 0;
        let mut skip_idx = -1;

        while input_idx < input_chars.len() {
            // if we have a match - advance pattern and string
            if pattern_idx < pattern_chars.len()
                && (input_chars[input_idx] == pattern_chars[pattern_idx]
                    || pattern_chars[pattern_idx] == '?')
            {
                input_idx += 1;
                pattern_idx += 1;

                continue;
            }

            // if we have * - compress all *'s into one
            // save position in pattern at which we found *
            // also save position in string at which we found * in pattern
            if pattern_idx < pattern_chars.len() && pattern_chars[pattern_idx] == '*' {
                skip_idx = pattern_idx as i32;
                current_match_idx = input_idx;
                pattern_idx += 1;

                continue;
            }

            // if we had * in pattern then advance to next position in string and continue checks
            if skip_idx > -1 {
                pattern_idx = (skip_idx + 1) as usize; // set this to start matching after * in pattern
                current_match_idx += 1;
                input_idx = current_match_idx;

                continue;
            }

            // no symbol matches and no * in pattern
            return false;
        }

        // if pattern ends on * - skip it
        while pattern_idx < pattern_chars.len() && pattern_chars[pattern_idx] == '*' {
            pattern_idx += 1;
        }

        // if we reached end of pattern - we matched a string
        pattern_idx == pattern_chars.len()
    }
}
