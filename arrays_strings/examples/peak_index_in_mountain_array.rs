/*
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

fn main() {
    println!("{}", peak_index_in_mountain_array(vec![0, 1, 0]));
    println!("{}", peak_index_in_mountain_array(vec![0, 2, 1, 0]));
}

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
