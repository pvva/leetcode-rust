/*
https://leetcode.com/problems/container-with-most-water/

You are given an integer array height of length n. There are n vertical lines drawn such that the
two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container contains
the most water.

Return the maximum amount of water a container can store.

Notice that you may not slant the container.

Example 1:

Input: height = [1,8,6,2,5,4,8,3,7]
Output: 49
Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case,
the max area of water the container can contain is 49.

Example 2:

Input: height = [1,1]
Output: 1

Constraints:

n == height.length
2 <= n <= 10^5
0 <= height[i] <= 10^4
 */

fn main() {}

struct Solution;

// Solution goes below.

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area: i32 = 0;
        let mut s: usize = 0;
        let mut f: usize = height.len() - 1;

        while s < f {
            let h = height[s].min(height[f]);
            area = area.max((f - s) as i32 * h);

            while height[s] <= h && s < f {
                s += 1;
            }
            while height[f] <= h && s < f {
                f -= 1;
            }
        }

        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
