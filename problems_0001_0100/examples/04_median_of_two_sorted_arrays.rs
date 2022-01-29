/*
https://leetcode.com/problems/median-of-two-sorted-arrays/

Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

The overall run time complexity should be O(log (m+n)).

Example 1:

Input: nums1 = [1,3], nums2 = [2]
Output: 2.00000
Explanation: merged array = [1,2,3] and median is 2.

Example 2:

Input: nums1 = [1,2], nums2 = [3,4]
Output: 2.50000
Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.

Constraints:

nums1.length == m
nums2.length == n
0 <= m <= 1000
0 <= n <= 1000
1 <= m + n <= 2000
-10^6 <= nums1[i], nums2[i] <= 10^6
 */

fn main() {}

struct Solution;

// Solution goes below

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l = nums1.len() + nums2.len();
        let target_cut = l / 2;

        let (n1, n2) = if nums1.len() > nums2.len() {
            (&nums2, &nums1)
        } else {
            (&nums1, &nums2)
        };

        let mut left: usize = 0;
        let mut right = n1.len();

        while left < right {
            let i1 = (left + right) / 2;
            let i2 = target_cut - i1;

            if n1[i1] < n2[i2 - 1] {
                left = i1 + 1;
            } else {
                right = i1;
            }
        }

        // left is last cut index of nums1
        // right is set to last cut of nums2
        right = target_cut - left;

        // values
        let last_cut1 = if left == 0 { i32::MIN } else { n1[left - 1] };
        let next_cut1 = if left == n1.len() { i32::MAX } else { n1[left] };
        let last_cut2 = if right == 0 { i32::MIN } else { n2[right - 1] };
        let next_cut2 = if right == n2.len() {
            i32::MAX
        } else {
            n2[right]
        };

        if l % 2 == 1 {
            i32::min(next_cut1, next_cut2) as f64
        } else {
            (i32::max(last_cut1, last_cut2) as f64 + i32::min(next_cut1, next_cut2) as f64) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![3, 4], vec![1, 2]),
            2.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![2, 5, 8, 9], vec![1, 3, 6]),
            5.0
        );
    }
}
