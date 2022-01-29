/*
https://leetcode.com/problems/add-two-numbers/

You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order,
and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example 1:

Input: l1 = [2,4,3], l2 = [5,6,4]
Output: [7,0,8]
Explanation: 342 + 465 = 807.

Example 2:

Input: l1 = [0], l2 = [0]
Output: [0]

Example 3:

Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
Output: [8,9,9,9,0,0,0,1]
*/

fn make_list_from_number_representation(numbers: Vec<i32>) -> Option<Box<ListNode>> {
    if numbers.is_empty() {
        return None;
    }
    let mut head = Box::new(ListNode::new(0));
    let mut l = &mut head;
    l.val = *numbers.first().unwrap();
    for number in numbers.into_iter().skip(1).collect::<Vec<i32>>() {
        l.next = Some(Box::new(ListNode::new(number)));
        l = l.next.as_mut().unwrap();
    }

    Some(head)
}

fn print_list(list: Option<Box<ListNode>>) {
    let mut h = list;
    let mut d = "";
    while let Some(l) = h {
        print!("{}{}", d, l.val);
        h = l.next;
        d = " -> ";
    }
}

struct AddTwoNumberTestCase {
    l1: Vec<i32>,
    l2: Vec<i32>,
    result: String,
}

fn main() {
    let cases = vec![
        AddTwoNumberTestCase {
            l1: vec![2, 4, 3],
            l2: vec![5, 6, 4],
            result: "708".to_string(),
        },
        AddTwoNumberTestCase {
            l1: vec![0],
            l2: vec![0],
            result: "0".to_string(),
        },
        AddTwoNumberTestCase {
            l1: vec![9, 9, 9, 9, 9, 9, 9],
            l2: vec![9, 9, 9, 9],
            result: "89990001".to_string(),
        },
    ];
    for case in cases {
        let l1 = make_list_from_number_representation(case.l1.clone());
        let l2 = make_list_from_number_representation(case.l2.clone());
        let res = Solution::add_two_numbers(l1, l2);
        print!(
            "{:?} + {:?} is expected to be {}, got ",
            case.l1, case.l2, case.result
        );
        print_list(res);
        println!()
    }
}

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
}

struct Solution;

// solution goes below

// impl Solution {
//     fn single_add_step(
//         l1: Option<Box<ListNode>>,
//         l2: Option<Box<ListNode>>,
//         carry: i32,
//     ) -> (
//         i32,
//         Box<ListNode>,
//         Option<Box<ListNode>>,
//         Option<Box<ListNode>>,
//     ) {
//         let mut v: i32 = carry;
//         let mut list1: Option<Box<ListNode>> = None;
//         let mut list2: Option<Box<ListNode>> = None;
//
//         if let Some(l1ptr) = l1 {
//             v += l1ptr.val;
//             list1 = l1ptr.next;
//         }
//         if let Some(l2ptr) = l2 {
//             v += l2ptr.val;
//             list2 = l2ptr.next;
//         }
//         let c = v / 10;
//
//         (c, Box::new(ListNode::new(v % 10)), list1, list2)
//     }
//
//     pub fn add_two_numbers(
//         l1: Option<Box<ListNode>>,
//         l2: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         if l1.is_none() || l2.is_none() {
//             return None;
//         }
//
//         let (mut c, mut head, mut list1, mut list2) = Solution::single_add_step(l1, l2, 0);
//         let mut l = &mut head;
//
//         while list1.is_some() || list2.is_some() {
//             let (c_t, node, l1_t, l2_t) = Solution::single_add_step(list1, list2, c);
//             c = c_t;
//             list1 = l1_t;
//             list2 = l2_t;
//
//             l.next = Some(node);
//             l = l.next.as_mut().unwrap();
//         }
//         if c > 0 {
//             l.next = Some(Box::new(ListNode::new(c)));
//         }
//
//         Some(head)
//     }
// }

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::next(&l1, &l2, 0)
    }

    fn next(
        l1: &Option<Box<ListNode>>,
        l2: &Option<Box<ListNode>>,
        c: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && c == 0 {
            return None;
        }
        let val = [l1, l2]
            .iter()
            .fold(c, |acc, cur| acc + cur.as_ref().map(|n| n.val).unwrap_or(0));
        Some(Box::new(ListNode {
            val: val % 10,
            next: Self::next(
                l1.as_ref().map(|n| &n.next).unwrap_or(&None),
                l2.as_ref().map(|n| &n.next).unwrap_or(&None),
                val / 10,
            ),
        }))
    }
}
