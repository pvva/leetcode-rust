/*
A permutation of an array of integers is an arrangement of its members into a sequence or linear order.

For example, for arr = [1,2,3], the following are all the permutations of arr:
[1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1].
The next permutation of an array of integers is the next lexicographically greater permutation of
its integer. More formally, if all the permutations of the array are sorted in one container according
to their lexicographical order, then the next permutation of that array is the permutation that follows
it in the sorted container. If such arrangement is not possible, the array must be rearranged as the
lowest possible order (i.e., sorted in ascending order).

For example, the next permutation of arr = [1,2,3] is [1,3,2].
Similarly, the next permutation of arr = [2,3,1] is [3,1,2].
While the next permutation of arr = [3,2,1] is [1,2,3] because [3,2,1] does not have a lexicographical larger rearrangement.
Given an array of integers nums, find the next permutation of nums.

The replacement must be in place and use only constant extra memory.

Example 1:

Input: nums = [1,2,3]
Output: [1,3,2]
Example 2:

Input: nums = [3,2,1]
Output: [1,2,3]
Example 3:

Input: nums = [1,1,5]
Output: [1,5,1]

Constraints:
1 <= nums.length <= 100
0 <= nums[i] <= 100
 */
fn main() {}

struct Solution;

// Solution goes bellow.

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        if nums.len() == 2 {
            let t = nums[0];
            nums[0] = nums[1];
            nums[1] = t;
            return;
        }
        let mut dec_idx = None;
        for i in (1..nums.len()).rev() {
            if nums[i] > nums[i - 1] {
                dec_idx = Some(i - 1);
                break;
            }
        }
        if let Some(d) = dec_idx {
            nums[d + 1..].reverse();
            for i in d + 1..nums.len() {
                if nums[i] > nums[d] {
                    nums[i] ^= nums[d];
                    nums[d] ^= nums[i];
                    nums[i] ^= nums[d];
                    break;
                }
            }
        } else {
            nums.reverse();
        }
    }

    pub fn next_permutation_idiomatic(nums: &mut Vec<i32>) {
        // find the index of the first element that is smaller than its next element going from the right
        let first_dec_idx = (1..nums.len())
            .rev()
            .find(|&i| nums[i - 1] < nums[i]);

        match first_dec_idx {
            Some(idx) => {
                // among elements satisfying index >= idx, find the smallest one that is larger than nums[idx - 1]
                let smaller_idx = (idx..nums.len()).fold(idx, |acc, i| {
                    if nums[i] <= nums[acc] && nums[i] > nums[idx - 1] {
                        i
                    } else {
                        acc
                    }
                });
                // swap nums[idx - 1] and nums[smaller_idx]
                nums[idx - 1] ^= nums[smaller_idx];
                nums[smaller_idx] ^= nums[idx - 1];
                nums[idx - 1] ^= nums[smaller_idx];
                // reverse the elements from idx to the end
                nums[idx..].reverse();
            }
            None => {
                nums.reverse();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<i32>,
        want: Vec<i32>
    }

    #[test]
    fn test_next_permutation() {
        let test_cases = vec![
            TestCase {
                input: vec![1, 2, 3],
                want: vec![1, 3, 2],
            },
            TestCase {
                input: vec![3, 2, 1],
                want: vec![1, 2, 3],
            },
            TestCase {
                input: vec![1, 1, 5],
                want: vec![1, 5, 1],
            },
            TestCase {
                input: vec![1, 3, 2],
                want: vec![2, 1, 3],
            },
        ];
        for tc in test_cases {
            let mut v = tc.input.clone();
            Solution::next_permutation(&mut v);
            assert_eq!(tc.want, v);
        }
    }
}
