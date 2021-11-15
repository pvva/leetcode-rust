/*
https://leetcode.com/problems/binary-tree-maximum-path-sum/

A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence
has an edge connecting them. A node can only appear in the sequence at most once. Note that the
path does not need to pass through the root.

The path sum of a path is the sum of the node's values in the path.

Given the root of a binary tree, return the maximum path sum of any non-empty path.

Example 1:

Input: root = [1,2,3]
Output: 6
Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.

Example 2:

Input: root = [-10,9,20,null,null,15,7]
Output: 42
Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.

Constraints:

The number of nodes in the tree is in the range [1, 3 * 10^4].
-1000 <= Node.val <= 1000
*/

fn main() {
    let mut nr = TreeNode::new(-10);
    let nrl = TreeNode::new(9);
    let mut nrr = TreeNode::new(20);
    let nrrl = TreeNode::new(15);
    let nrrr = TreeNode::new(7);

    nrr.left = Some(Rc::new(RefCell::new(nrrl)));
    nrr.right = Some(Rc::new(RefCell::new(nrrr)));
    nr.left = Some(Rc::new(RefCell::new(nrl)));
    nr.right = Some(Rc::new(RefCell::new(nrr)));
    let root = Some(Rc::new(RefCell::new(nr)));

    println!("{}", Solution::max_path_sum(root));
}

struct Solution;

// solution is below

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (m, cm) = Self::mps(&root);
        Self::max(vec![m, cm])
    }

    // returns (max path value that can be continued, max path value that ends on this node or close to this node)
    fn mps(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        let mut m = i32::MIN;
        let mut cm = i32::MIN;

        if let Some(n) = node {
            let borrowed = n.borrow();
            let v = &borrowed.val;

            let (lm, lcm) = Self::mps(&borrowed.left);
            let (rm, rcm) = Self::mps(&borrowed.right);
            let mut lm2 = lm;
            if lm2 == std::i32::MIN {
                lm2 = 0;
            }
            let mut rm2 = rm;
            if rm2 == std::i32::MIN {
                rm2 = 0;
            }

            m = Self::max(vec![*v, lm2 + *v, rm2 + *v]);
            cm = Self::max(vec![lm, rm, *v, lcm, rcm, lm2 + rm2 + *v]);
        }

        (m, cm)
    }

    fn max(t: Vec<i32>) -> i32 {
        let mut m = i32::MIN;
        for v in t.iter() {
            if *v > m {
                m = *v;
            }
        }
        m
    }
}
