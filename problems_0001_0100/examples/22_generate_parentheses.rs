/*
https://leetcode.com/problems/generate-parentheses/

Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

 Example 1:

Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]

Example 2:

Input: n = 1
Output: ["()"]

Constraints:

1 <= n <= 8
 */
fn main() {
    println!("{}", Solution::generate_parenthesis(1).join(", "));
    println!("{}", Solution::generate_parenthesis(2).join(", "));
    println!("{}", Solution::generate_parenthesis(3).join(", "));
}

struct Solution;

// Solution goes bellow

impl Solution {
    // pub fn generate_parenthesis(n: i32) -> Vec<String> {
    //     Solution::gen_parens(n, n, vec![])
    // }
    //
    // pub fn gen_parens(open: i32, closed: i32, state: Vec<String>) -> Vec<String> {
    //     if open == 0 && closed == 0 {
    //         return state;
    //     }
    //
    //     let mut r: Vec<String> = vec![];
    //     if open > 0 {
    //         let t: Vec<String>;
    //         if state.is_empty() {
    //             t = vec!["(".to_string()];
    //         } else {
    //             t = state
    //                 .iter()
    //                 .map(|s| vec![(*s).clone(), "(".to_string()].join(""))
    //                 .collect();
    //         }
    //         r = Solution::gen_parens(open - 1, closed, t);
    //     }
    //     if closed > open {
    //         let t = state
    //             .into_iter()
    //             .map(|s| vec![s, ")".to_string()].join(""))
    //             .collect();
    //         for s in Solution::gen_parens(open, closed - 1, t).into_iter() {
    //             r.push(s);
    //         }
    //     }
    //
    //     r
    // }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn back_track(s: String, open: i32, closed: i32) -> Vec<String> {
            if open == 0 && closed == 0 {
                return vec![s];
            }
            let mut res = vec![];
            if open > 0 {
                res.append(&mut back_track(s.clone() + "(", open - 1, closed + 1));
            }
            if closed > 0 {
                res.append(&mut back_track(s.clone() + ")", open, closed - 1));
            }
            res
        }
        back_track("".to_string(), n, 0)
    }
}
