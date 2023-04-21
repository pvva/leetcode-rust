/*
https://leetcode.com/problems/3sum/

Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0?
Find all unique triplets in the array which gives the sum of zero.

Note:

The solution set must not contain duplicate triplets.

Example:

Given array nums = [-1, 0, 1, 2, -1, -4],

A solution set is:
[
  [-1, 0, 1],
  [-1, -1, 2]
]
*/

fn main() {}

struct Solution;

// Solution goes below

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0 as i32; 0]; 0];

        let mut n = nums;
        n.sort();

        for (i, _) in n.iter().enumerate() {
            if i > 0 && n[i - 1] == n[i] {
                continue;
            }

            let mut si = i + 1;
            let mut ti = n.len() - 1;

            while si < ti {
                let s = n[i] + n[si] + n[ti];
                if s == 0 {
                    res.push(vec![n[i], n[si], n[ti]]);
                    si += 1;
                    ti -= 1;
                    while si < ti && n[si - 1] == n[si] {
                        si += 1;
                    }
                    while si < ti && n[ti + 1] == n[si] {
                        ti -= 1;
                    }
                } else if s < 0 {
                    si += 1;
                } else {
                    ti -= 1;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(Solution::three_sum(vec![]), vec![vec![0 as i32; 0]; 0]);
        assert_eq!(Solution::three_sum(vec![0]), vec![vec![0 as i32; 0]; 0]);
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
}
