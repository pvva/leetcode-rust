/*
Given a string containing just the characters '(' and ')', return the length of the longest valid
(well-formed) parentheses substring.

Example 1:

Input: s = "(()"
Output: 2
Explanation: The longest valid parentheses substring is "()".
Example 2:

Input: s = ")()())"
Output: 4
Explanation: The longest valid parentheses substring is "()()".
Example 3:

Input: s = ""
Output: 0

Constraints:

0 <= s.length <= 3 * 104
s[i] is '(', or ')'.
 */
fn main() {}

struct Solution;

// Solution goes bellow.

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.len() <= 1 {
            return 0;
        }
        let chars: Vec<char> = s.chars().collect();
        // collection of broken sequences start indices
        // between these indices valid sequences are located
        let mut broken: Vec<usize> = vec![];
        for i in 0..chars.len() {
            if chars[i] == '(' {
                broken.push(i);
            } else if broken.is_empty() {
                broken.push(i);
            } else {
                let lb = broken.len() - 1;
                if chars[broken[lb]] == '(' {
                    // close opened broken sequence
                    broken.pop();
                } else {
                    broken.push(i);
                }
            }
        }
        if broken.is_empty() {
            return s.len() as i32;
        }
        broken.push(s.len());
        let mut res: usize = broken[0];
        for i in 1..broken.len() {
            // need to subtract 1 because sequences are between indices and not starting and ending
            // at them.
            res = usize::max(res, broken[i] - broken[i - 1] - 1);
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<'a> {
        input: &'a str,
        want: i32
    }

    #[test]
    fn test_longest_valid_parentheses() {
        let test_cases = vec![
            TestCase {
                input: "(()",
                want: 2,
            },
            TestCase {
                input: ")()())",
                want: 4,
            },
            TestCase {
                input: "",
                want: 0,
            },
            TestCase {
                input: "))",
                want: 0,
            },
            TestCase {
                input: "()(()",
                want: 2,
            },
            TestCase {
                input: "(()(((()",
                want: 2,
            },
            TestCase {
                input: "()()",
                want: 4,
            },
            TestCase {
                input: "())",
                want: 2,
            },
        ];
        for tc in test_cases {
            assert_eq!(tc.want, Solution::longest_valid_parentheses(tc.input.to_string()));
        }
    }
}
