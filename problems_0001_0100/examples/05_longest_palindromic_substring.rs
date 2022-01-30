/*
https://leetcode.com/problems/longest-palindromic-substring/

Given a string s, return the longest palindromic substring in s.

Example 1:

Input: s = "babad"
Output: "bab"
Explanation: "aba" is also a valid answer.

Example 2:

Input: s = "cbbd"
Output: "bb"

Constraints:

1 <= s.length <= 1000
s consist of only digits and English letters.
*/

fn main() {}

struct Solution;

// Solution goes below

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let l = s.len();
        if l <= 1 {
            return s;
        }
        let chars: Vec<_> = s.bytes().collect();

        let mut max_len: usize = 1;
        let mut max_s = &chars[0..1];

        for il in 2..=3 as usize {
            for i in 0..(l - il + 1) {
                if !Solution::is_palindrome(&chars[i..i + il]) {
                    continue;
                }

                if max_len < il {
                    max_len = il;
                    max_s = &chars[i..i + il];
                }

                for j in 1..=usize::min(i, l - i - il) {
                    if chars[i - j] != chars[i + il - 1 + j] {
                        break;
                    }
                    let c_len = il + 2 * j - 1;
                    if max_len < c_len {
                        max_len = c_len;
                        max_s = &chars[i - j..i + il + j];
                    }
                }
            }
        }

        String::from_utf8(max_s.to_vec()).unwrap()
    }

    fn is_palindrome(chars: &[u8]) -> bool {
        for i in 0..(chars.len() / 2) {
            if chars[i] != chars[chars.len() - 1 - i] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        // assert_eq!(
        //     Solution::longest_palindrome("babad".to_string()),
        //     "bab".to_string()
        // );
        // assert_eq!(
        //     Solution::longest_palindrome("cbbd".to_string()),
        //     "bb".to_string()
        // );
        assert_eq!(
            Solution::longest_palindrome("babadada".to_string()),
            "adada".to_string()
        );
    }
}
