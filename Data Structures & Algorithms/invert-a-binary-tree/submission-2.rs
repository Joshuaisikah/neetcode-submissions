// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = VecDeque::new();
        if let Some(r) = &root {
            queue.push_back(r.clone());
        }
        while let Some(node) = queue.pop_front() {
    let mut guard = node.borrow_mut();
    let n = &mut *guard; // now a plain &mut TreeNode, disjoint field borrows work fine
    std::mem::swap(&mut n.left, &mut n.right);
    if let Some(l) = &n.left { queue.push_back(l.clone()); }
    if let Some(r) = &n.right { queue.push_back(r.clone()); }
}

        root
    }
}

    