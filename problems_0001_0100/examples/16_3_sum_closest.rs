/*
https://leetcode.com/problems/3sum-closest/

Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.

Return the sum of the three integers.

You may assume that each input would have exactly one solution.


Example 1:

Input: nums = [-1,2,1,-4], target = 1
Output: 2
Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).

Example 2:

Input: nums = [0,0,0], target = 1
Output: 0

Constraints:

3 <= nums.length <= 1000
-1000 <= nums[i] <= 1000
-10^4 <= target <= 10^4
*/

fn main() {}

struct Solution;

// Solution goes below

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 3 {
            return nums.iter().fold(0, |stor, v| stor + v)
        }
        nums.sort();
        let mut result = 0;
        let mut diff = i32::MAX;
        for (idx, v) in nums.iter().enumerate() {
            if idx > 0 && *v == nums[idx - 1] {
                continue
            }
            let mut si = idx +1;
            let mut ti = nums.len() - 1;
            while si < ti {
                let sum = *v + nums[si] + nums[ti];
                let mut d = sum - target;
                if d == 0 {
                    return sum;
                } else if d < 0 {
                    si += 1;
                    d = -d;
                } else {
                    ti -= 1;
                }
                if d < diff {
                    diff = d;
                    result = sum;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_closest() {
        assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
        assert_eq!(0, Solution::three_sum_closest(vec![0, 0, 0], 1));
        assert_eq!(3, Solution::three_sum_closest(vec![1, 1, -1, -1, 3], 3));
        assert_eq!(3, Solution::three_sum_closest(vec![1, 1, 1, 1, 1, 1], 3));
    }
}
