/*
https://leetcode.com/problems/reverse-nodes-in-k-group/

Given a linked list, reverse the nodes of a linked list k at a time and return its modified list.

k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is
not a multiple of k then left-out nodes, in the end, should remain as it is.

You may not alter the values in the list's nodes, only nodes themselves may be changed.

Example 1:

Input: head = [1,2,3,4,5], k = 2
Output: [2,1,4,3,5]

Example 2:

Input: head = [1,2,3,4,5], k = 3
Output: [3,2,1,4,5]

Example 3:

Input: head = [1,2,3,4,5], k = 1
Output: [1,2,3,4,5]

Example 4:

Input: head = [1], k = 1
Output: [1]

Constraints:

The number of nodes in the list is in the range sz.
1 <= sz <= 5000
0 <= Node.val <= 1000
1 <= k <= sz

Follow-up: Can you solve the problem in O(1) extra memory space?
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
    fn reverse_vec(vec: &mut Vec<i32>, k: usize) {
        let half = k>>1;
        for i in (0..vec.len() - vec.len() % k).step_by(k as usize) {
            for j in 0..half {
                vec[i + j] ^= vec[i + k - j - 1];
                vec[i + k - j - 1] ^= vec[i + j];
                vec[i + j] ^= vec[i + k - j - 1];
            }
        }
    }

    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 {
            return head;
        }
        let mut vec: Vec<i32> = vec![];
        while let Some(node) = head.take() {
            vec.push(node.val);
            head = node.next;
        }
        Solution::reverse_vec(&mut vec, k as usize);

        let mut h: Box<ListNode> = Box::new(ListNode::new(vec[0]));
        let mut c = &mut h;
        for v in vec.into_iter().skip(1).collect::<Vec<i32>>() {
            c.next = Some(Box::new(ListNode::new(v)));
            c = c.next.as_mut().unwrap();
        }
        Some(h)
    }

    pub fn reverse_k_group_two_lists(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut tmp_head = ListNode::new(0);
        let mut last_node = &mut tmp_head;

        'outer:
        loop {
            // reverse group
            for _ in 0..k {
                if let Some(mut node) = head.take() {
                    // get next elem and store it for the upcoming iteration
                    head = node.next.take();
                    // put existing chain to the end of the current element.
                    node.next = last_node.next.take();
                    // make current node a head of the chain.
                    last_node.next = Some(node);
                } else {
                    break 'outer;
                }
            }
            // move the head of the chain to the end of the chain via borrow unwrap.
            while let Some(ref mut next) = last_node.next {
                last_node = next;
            }
        };

        // if there's a group with the length less then k - reverse it back.
        let mut last_head = ListNode::new(0);
        while let Some(mut node) = last_node.next.take() {
            last_node.next = node.next.take();
            node.next = last_head.next.take();
            last_head.next = Some(node);
        }
        last_node.next = last_head.next.take();
        tmp_head.next.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_k_group_1() {
        let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        // let result = Solution::reverse_k_group(Some(list), 2).unwrap();
        let result = Solution::reverse_k_group_two_lists(Some(list), 2).unwrap();
        assert_eq!("2 -> 1 -> 4 -> 3 -> 5", result.to_string())
    }

    #[test]
    fn test_reverse_k_group_2() {
        let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        // let result = Solution::reverse_k_group(Some(list), 3).unwrap();
        let result = Solution::reverse_k_group_two_lists(Some(list), 3).unwrap();
        assert_eq!("3 -> 2 -> 1 -> 4 -> 5", result.to_string())
    }

    #[test]
    fn test_reverse_k_group_3() {
        let list = ListNode::from_vec(vec![1, 2]);
        // let result = Solution::reverse_k_group(Some(list), 1).unwrap();
        let result = Solution::reverse_k_group_two_lists(Some(list), 1).unwrap();
        assert_eq!("1 -> 2", result.to_string())
    }

    #[test]
    fn test_reverse_k_group_4() {
        let list = ListNode::from_vec(vec![1, 2]);
        // let result = Solution::reverse_k_group(Some(list), 2).unwrap();
        let result = Solution::reverse_k_group_two_lists(Some(list), 2).unwrap();
        assert_eq!("2 -> 1", result.to_string())
    }
}
