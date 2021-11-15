/*
https://leetcode.com/problems/array-partition-i/

Given an array of 2n integers, your task is to group these integers into n pairs of integer,
say (a1, b1), (a2, b2), ..., (an, bn) which makes sum of min(ai, bi) for all i from 1 to n as large as possible.

Example 1:

Input: nums = [1,4,3,2]
Output: 4
Explanation: All possible pairings (ignoring the ordering of elements) are:
1. (1, 4), (2, 3) -> min(1, 4) + min(2, 3) = 1 + 2 = 3
2. (1, 3), (2, 4) -> min(1, 3) + min(2, 4) = 1 + 2 = 3
3. (1, 2), (3, 4) -> min(1, 2) + min(3, 4) = 1 + 3 = 4
So the maximum possible sum is 4.

Example 2:

Input: nums = [6,2,6,5,1,2]
Output: 9
Explanation: The optimal pairing is (2, 1), (2, 5), (6, 6). min(2, 1) + min(2, 5) + min(6, 6) = 1 + 2 + 6 = 9.

Constraints:

1 <= n <= 10^4
nums.length == 2 * n
-10^4 <= nums[i] <= 10^4
*/

fn main() {
    println!("{}", array_pair_sum(vec![1, 4, 3, 2]))
}

fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut ns = nums;
    ns.sort();
    // ns.iter().step_by(2).sum() // this is slower then the following code
    let mut result: i32 = 0;
    for i in 0..ns.len() >> 1 {
        result += ns[i << 1];
    }
    result
}
