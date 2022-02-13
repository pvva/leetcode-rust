/*
https://leetcode.com/problems/remove-nth-node-from-end-of-list/

Given the head of a linked list, remove the nth node from the end of the list and return its head.

Example 1:

Input: head = [1,2,3,4,5], n = 2
Output: [1,2,3,5]

Example 2:

Input: head = [1], n = 1
Output: []

Example 3:

Input: head = [1,2], n = 1
Output: [1]

Constraints:

The number of nodes in the list is sz.
1 <= sz <= 30
0 <= Node.val <= 100
1 <= n <= sz
 */
fn main() {
    let l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);

    println!("{}", l.clone().unwrap().to_str());
    println!("{}", Solution::remove_nth_from_end(l, 2).unwrap().to_str());
}

struct Solution;

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

// Solution goes bellow

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if n == 0 {
            return head;
        }
        let mut s_head = Box::new(ListNode { val: 0, next: head });
        let mut fast = s_head.clone();
        let mut slow = s_head.as_mut();

        for _ in 0..n {
            fast = fast.next.unwrap();
        }
        while let Some(node) = fast.next {
            slow = slow.next.as_mut().unwrap();
            fast = node;
        }
        slow.next = slow.next.as_mut().unwrap().next.clone();

        s_head.next
    }
}
