/*
Given a non-empty binary tree, find the maximum path sum.

For this problem, a path is defined as any sequence of nodes from some starting node to any node in
the tree along the parent-child connections. The path must contain at least one node and does not
need to go through the root.
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
        let mut m = std::i32::MIN;
        let mut cm = std::i32::MIN;

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
        let mut m = std::i32::MIN;
        for v in t.iter() {
            if *v > m {
                m = *v;
            }
        }
        m
    }
}
