/*
https://leetcode.com/problems/letter-combinations-of-a-phone-number/

Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could
represent. Return the answer in any order.

Example 1:

Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]

Example 2:

Input: digits = ""
Output: []

Example 3:

Input: digits = "2"
Output: ["a","b","c"]


Constraints:

0 <= digits.length <= 4
digits[i] is a digit in the range ['2', '9'].
*/

fn main() {}

struct Solution;

// Solution goes bellow.

const DIAL_DIGITS: &[&str] = &["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for b in digits.bytes() {
            // would be nice to check boundaries, but... :)
            let letters = DIAL_DIGITS[b as usize - '0' as usize];
            if result.is_empty() {
                for l in (*letters).chars() {
                    result.push(l.to_string());
                }
            } else {
                let mut r: Vec<String> = vec![];
                for s in result {
                    for l in (*letters).chars() {
                        let mut t = s.clone();
                        t.push_str(l.to_string().as_str());
                        r.push(t);
                    }
                }
                result = r;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strvec(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|v| v.to_string()).collect()
    }

    #[test]
    fn test_letter_combinations() {
        assert_eq!(strvec(vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]),
                   Solution::letter_combinations("23".to_string()));
        assert_eq!(strvec(vec![]),
                   Solution::letter_combinations("".to_string()));
        assert_eq!(strvec(vec!["a", "b", "c"]),
                   Solution::letter_combinations("2".to_string()));
    }
}