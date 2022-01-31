/*
https://leetcode.com/problems/string-to-integer-atoi/

Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).

The algorithm for myAtoi(string s) is as follows:

Read in and ignore any leading whitespace.
Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either.
This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0.
Change the sign as necessary (from step 2).
If the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1], then clamp the integer so that it remains in the range.
Specifically, integers less than -2^31 should be clamped to -2^31, and integers greater than 2^31 - 1 should be clamped to 2^31 - 1.
Return the integer as the final result.

Note:

Only the space character ' ' is considered a whitespace character.
Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.

Example 1:

Input: s = "42"
Output: 42
Explanation: The underlined characters are what is read in, the caret is the current reader position.
Step 1: "42" (no characters read because there is no leading whitespace)
         ^
Step 2: "42" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "42" ("42" is read in)
           ^
The parsed integer is 42.
Since 42 is in the range [-2^31, 2^31 - 1], the final result is 42.

Example 2:

Input: s = "   -42"
Output: -42
Explanation:
Step 1: "   -42" (leading whitespace is read and ignored)
            ^
Step 2: "   -42" ('-' is read, so the result should be negative)
             ^
Step 3: "   -42" ("42" is read in)
               ^
The parsed integer is -42.
Since -42 is in the range [-2^31, 2^31 - 1], the final result is -42.

Example 3:

Input: s = "4193 with words"
Output: 4193

Explanation:
Step 1: "4193 with words" (no characters read because there is no leading whitespace)
         ^
Step 2: "4193 with words" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "4193 with words" ("4193" is read in; reading stops because the next character is a non-digit)
             ^
The parsed integer is 4193.
Since 4193 is in the range [-2^31, 2^31 - 1], the final result is 4193.

Constraints:

0 <= s.length <= 200
s consists of English letters (lower-case and upper-case), digits (0-9), ' ', '+', '-', and '.'.
 */
fn main() {}

struct Solution;

// Solution goes below

// impl Solution {
//     pub fn my_atoi(s: String) -> i32 {
//         let bytes: Vec<_> = s.trim_start().bytes().collect();
//
//         let mut res: i32 = 0;
//         let mut sign: i64 = 0;
//
//         for (_, b) in bytes.iter().enumerate() {
//             let is_digit = *b >= b'0' && *b <= b'9';
//             if sign == 0 {
//                 if *b == b'+' {
//                     sign = 1;
//                 } else if *b == b'-' {
//                     sign = -1;
//                 } else if is_digit {
//                     sign = 1;
//                     res = (*b - b'0') as i32;
//                 } else {
//                     break;
//                 }
//             } else if is_digit {
//                 let t: i64 = res as i64 * 10 + sign * (*b - b'0') as i64;
//                 if t > i32::MAX as i64 {
//                     res = i32::MAX;
//                     break;
//                 } else if t < i32::MIN as i64 {
//                     res = i32::MIN;
//                     break;
//                 } else {
//                     res = t as i32;
//                 }
//             } else {
//                 break;
//             }
//         }
//
//         res
//     }
// }

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let s_ = str.trim_start();
        let (n, s) = match s_.chars().take(1).next() {
            Some('+') => (1, 1),
            Some(x) if x.is_digit(10) => (0, 1),
            Some('-') => (1, -1),
            _ => return 0,
        };
        let mut res: i32 = 0;
        let overflow = if s > 0 { i32::MAX } else { i32::MIN };
        for c in s_.chars().skip(n).take_while(|x| x.is_digit(10)) {
            let (r, o) = res.overflowing_mul(10);
            if o {
                return overflow;
            }
            let (r, o) = r.overflowing_add(s * (c as i32 - '0' as i32));
            if o {
                return overflow;
            }
            res = r;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("    -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("2147483648".to_string()), 2147483647);
        assert_eq!(Solution::my_atoi("00000-42a1234".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }
}
