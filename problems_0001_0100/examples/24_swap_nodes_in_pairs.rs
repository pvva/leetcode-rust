/*
https://leetcode.com/problems/swap-nodes-in-pairs/

Given a linked list, swap every two adjacent nodes and return its head. You must solve the problem without
modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)

Example 1:

Input: head = [1,2,3,4]
Output: [2,1,4,3]

Example 2:

Input: head = []
Output: []

Example 3:

Input: head = [1]
Output: [1]
*/

fn main() {}

struct Solution;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    fn to_string(&self) -> String {
        let tail = match &self.next {
            None => { "".to_string() }
            Some(node) => { format!(" -> {}", node.to_string()) }
        };
        format!("{}{}", self.val, tail)
    }

    fn from_vec(vec: Vec<i32>) -> Box<ListNode> {
        if vec.is_empty() {
            return Box::default();
        }
        let mut head: Box<ListNode> = Box::new(ListNode::new(vec[0]));
        let mut c = &mut head;
        for v in vec.into_iter().skip(1).collect::<Vec<i32>>() {
            c.next = Some(Box::new(ListNode::new(v)));
            c = c.next.as_mut().unwrap();
        }
        head
    }
}

// Solution goes bellow.

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tmp_head = Box::new(ListNode{val: 0, next: head});

        let mut curr_ptr = &mut tmp_head;
        while let Some(mut next) = curr_ptr.next.take() {
            if let Some(mut next_next) = next.next.take() {
                let tail = next_next.next.take();
                next.next = tail;
                next_next.next = Some(next);
                curr_ptr.next = Some(next_next);
                curr_ptr = curr_ptr.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                curr_ptr.next = Some(next);
                break;
            }
        }

        tmp_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_init() {
        assert_eq!(ListNode::from_vec(vec![1, 3, 5]).to_string(), "1 -> 3 -> 5")
    }

    #[test]
    fn test_swap_pairs_1() {
        let list = ListNode::from_vec(vec![1, 2, 3, 4]);
        let result = Solution::swap_pairs(Some(list)).unwrap();
        assert_eq!("2 -> 1 -> 4 -> 3", result.to_string());
    }

    #[test]
    fn test_swap_pairs_2() {
        let list = ListNode::from_vec(vec![1]);
        let result = Solution::swap_pairs(Some(list)).unwrap();
        assert_eq!("1", result.to_string());
    }
}
