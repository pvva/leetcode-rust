/*
Given an array of 2n integers, your task is to group these integers into n pairs of integer,
say (a1, b1), (a2, b2), ..., (an, bn) which makes sum of min(ai, bi) for all i from 1 to n as large as possible.

Example 1:
Input: [1,4,3,2]

Output: 4
Explanation: n is 2, and the maximum sum of pairs is 4 = min(1, 2) + min(3, 4).
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
