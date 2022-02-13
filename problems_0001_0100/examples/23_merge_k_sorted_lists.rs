/*
https://leetcode.com/problems/merge-k-sorted-lists/

You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.

Merge all the linked-lists into one sorted linked-list and return it.

Example 1:

Input: lists = [[1,4,5],[1,3,4],[2,6]]
Output: [1,1,2,3,4,4,5,6]
Explanation: The linked-lists are:
[
  1->4->5,
  1->3->4,
  2->6
]
merging them into one sorted list:
1->1->2->3->4->4->5->6

Example 2:

Input: lists = []
Output: []
Example 3:

Input: lists = [[]]
Output: []

Constraints:

k == lists.length
0 <= k <= 10^4
0 <= lists[i].length <= 500
-10^4 <= lists[i][j] <= 10^4
lists[i] is sorted in ascending order.
The sum of lists[i].length will not exceed 10^4.
 */
fn main() {
    let l1 = ListNode::from_vec(vec![1, 4, 5]);
    let l2 = ListNode::from_vec(vec![1, 3, 4]);
    let l3 = ListNode::from_vec(vec![2, 6]);

    let r = Solution::merge_k_lists(vec![l1, l2, l3]);
    println!("{}", r.unwrap().to_str());
}

struct Solution;

// Solution goes below

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn to_str(&self) -> String {
        let mut l: Vec<String> = vec![self.val.to_string()];

        let mut n = &self.next;
        while let Some(node) = n {
            l.push(node.val.to_string());

            n = &(n.as_ref().unwrap().next);
        }

        l.join(" -> ")
    }

    fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut node: Option<Box<ListNode>> = None;

        for v in vec.iter().rev() {
            node = Some(Box::new(ListNode {
                val: *v,
                next: node.clone(),
            }));
        }

        node
    }
}

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

impl PartialOrd<Self> for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for l in lists {
            heap.push(Reverse(l));
        }

        let mut node: Box<ListNode> = Box::new(ListNode::new(0));
        let mut n = node.as_mut();

        while let Some(Reverse(r)) = heap.pop() {
            if let Some(l) = r {
                n.next = Some(l.clone());
                n = n.next.as_mut().unwrap();
                heap.push(Reverse(l.next));
            }
        }

        node.next
    }
}
