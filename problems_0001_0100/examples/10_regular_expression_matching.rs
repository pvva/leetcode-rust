/*
https://leetcode.com/problems/regular-expression-matching/

Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:

'.' Matches any single character.
'*' Matches zero or more of the preceding element.
The matching should cover the entire input string (not partial).

 Example 1:

Input: s = "aa", p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".

Example 2:

Input: s = "aa", p = "a*"
Output: true
Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".

Example 3:

Input: s = "ab", p = ".*"
Output: true
Explanation: ".*" means "zero or more (*) of any character (.)".

Constraints:

1 <= s.length <= 20
1 <= p.length <= 30
s contains only lowercase English letters.
p contains only lowercase English letters, '.', and '*'.
It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
 */

fn main() {}

struct Solution;

// Solution goes below

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let sc: Vec<_> = s.chars().collect();
        let pc: Vec<_> = p.chars().collect();
        Solution::is_match_chars(&sc[..], &pc[..])
    }

    fn is_match_chars(s: &[char], p: &[char]) -> bool {
        if p.len() == 0 {
            return s.len() == 0;
        }

        if s.len() == 0 {
            if p.len() > 1 && p[1] == '*' {
                return Solution::is_match_chars(s, &p[2..]);
            }

            return false;
        }

        if s[0] != p[0] && p[0] != '.' {
            if p.len() > 1 && p[1] == '*' {
                return Solution::is_match_chars(s, &p[2..]);
            }

            return false;
        }

        if p.len() > 1 && p[1] == '*' {
            return Solution::is_match_chars(s, &p[2..]) || Solution::is_match_chars(&s[1..], p);
        }

        Solution::is_match_chars(&s[1..], &p[1..])
    }

    pub fn is_match_dp(s: String, p: String) -> bool {
        let sc: Vec<_> = s.chars().collect();
        let pc: Vec<_> = p.chars().collect();

        let mut m = vec![vec![false; pc.len() + 1]; sc.len() + 1];
        m[0][0] = true;

        for i in 1..=sc.len() {
            let c = sc[i - 1]; // current char
            for j in 1..=pc.len() {
                let t = pc[j - 1]; // current pattern value

                if t == c || t == '.' {
                    // advance current match process from previous positions in s & p
                    m[i][j] = m[i - 1][j - 1]
                } else if t == '*' {
                    // drag matching for initial "<char>*" sequence
                    m[0][j] = m[0][j - 2];
                    // advance match for pattern value -2 positions (because * follows matcher char)
                    m[i][j] = m[i][j - 2];

                    let _t = pc[j - 2]; // matcher char
                    if _t == '.' || _t == c {
                        // match previous char for current matcher char
                        m[i][j] = m[i][j] || m[i - 1][j];
                    }
                }
            }
        }

        m[sc.len()][pc.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
            false
        );
    }

    #[test]
    fn test_is_match_dp() {
        assert_eq!(
            Solution::is_match_dp("aa".to_string(), "a".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match_dp("aa".to_string(), "a*".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match_dp("ab".to_string(), ".*".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match_dp("ab".to_string(), ".*c".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match_dp("aab".to_string(), "c*a*b".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match_dp("mississippi".to_string(), "mis*is*p*.".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match_dp("a".to_string(), ".*..a*".to_string()),
            false
        );
    }
}
