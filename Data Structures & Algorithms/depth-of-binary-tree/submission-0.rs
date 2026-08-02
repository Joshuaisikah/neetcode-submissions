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

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        if let Some(r) = root {
            queue.push_back(r);
        }

        let mut level = 0;
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                if let Some(ref left) = node.left {
                    queue.push_back(left.clone());
                }
                if let Some(ref right) = node.right {
                    queue.push_back(right.clone());
                }
            }
            level += 1;
        }
        level
    }
}