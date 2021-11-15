/*
https://leetcode.com/problems/peak-index-in-a-mountain-array/

Let's call an array A a mountain if the following properties hold:

A.length >= 3
There exists some 0 < i < A.length - 1 such that A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1]
Given an array that is definitely a mountain, return any i such that
A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1].

Example 1:

Input: [0,1,0]
Output: 1
Example 2:

Input: [0,2,1,0]
Output: 1
*/

struct PeakIndexInMountainArrayTestCase {
    array: Vec<i32>,
    result: i32,
}

fn new_peak_index_in_mountain_array_case(a: Vec<i32>, r: i32) -> PeakIndexInMountainArrayTestCase {
    PeakIndexInMountainArrayTestCase {
        array: a,
        result: r,
    }
}

fn main() {
    let cases = vec![
        new_peak_index_in_mountain_array_case(vec![0, 1, 0], 1),
        new_peak_index_in_mountain_array_case(vec![0, 2, 1, 0], 1),
    ];

    for case in cases {
        println!(
            "for input {:?} expected result is {}, got {}",
            case.array.clone(),
            case.result,
            Solution::peak_index_in_mountain_array(case.array)
        )
    }
}

struct Solution;

// solution is below

impl Solution {
    fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let mut p = a[0];
        for i in 1..a.len() {
            if a[i] > p {
                p = a[i];
            } else {
                return (i - 1) as i32;
            }
        }
        0
    }
}
