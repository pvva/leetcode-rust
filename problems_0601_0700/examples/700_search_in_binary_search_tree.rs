/*
https://leetcode.com/problems/search-in-a-binary-search-tree/

Given the root node of a binary search tree (BST) and a value. You need to find the node in the BST
that the node's value equals the given value. Return the subtree rooted with that node. If such
node doesn't exist, you should return NULL.

For example,

Given the tree:
        4
       / \
      2   7
     / \
    1   3

And the value to search: 2
You should return this subtree:

      2
     / \
    1   3
In the example above, if we want to search the value 5, since there is no node with value 5, we should return NULL.

Note that an empty tree is represented by NULL, therefore you would see the expected output (serialized tree format)
as [], not null.

Constraints:
The number of nodes in the tree is in the range [1, 5000].
1 <= Node.val <= 10^7
root is a binary search tree.
1 <= val <= 10^7
*/

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let n1 = TreeNode::new(1);
    let mut n2 = TreeNode::new(2);
    let n3 = TreeNode::new(3);
    let mut n4 = TreeNode::new(4);
    let n7 = TreeNode::new(7);

    n2.left = Some(Rc::new(RefCell::new(n1)));
    n2.right = Some(Rc::new(RefCell::new(n3)));
    n4.left = Some(Rc::new(RefCell::new(n2)));
    n4.right = Some(Rc::new(RefCell::new(n7)));
    let root = Some(Rc::new(RefCell::new(n4)));

    //    println!("{:?}", root);
    println!("{:?}", search_bst(root, 2));
}

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

fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut current = root;
    while current != None {
        if let Some(node) = current {
            let borrowed = node.borrow();
            let v = &borrowed.val;
            if *v == val {
                return Some(node.clone());
            } else if val < *v {
                current = borrowed.left.clone();
            } else {
                current = borrowed.right.clone();
            }
        }
    }
    None
}
