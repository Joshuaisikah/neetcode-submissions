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

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut node = root.clone();
        let mut last: Option<Rc<RefCell<TreeNode>>> = None;
        let mut depths: HashMap<*const RefCell<TreeNode>, i32> = HashMap::new();

        while !stack.is_empty() || node.is_some() {
            if let Some(n) = node {
                stack.push(n.clone());
                node = n.borrow().left.clone();
            } else {
                let current = stack.last().unwrap().clone();
                let current_ref = current.borrow();
                let right_matches = match (&current_ref.right, &last) {
                    (None, _) => true,
                    (Some(r), Some(l)) => Rc::ptr_eq(r, l),
                    _ => false,
                };
                if right_matches {
                    drop(current_ref);
                    stack.pop();
                    let current_ref = current.borrow();
                    let left_depth = current_ref.left.as_ref()
                        .map_or(0, |l| *depths.get(&Rc::as_ptr(l)).unwrap_or(&0));
                    let right_depth = current_ref.right.as_ref()
                        .map_or(0, |r| *depths.get(&Rc::as_ptr(r)).unwrap_or(&0));
                    if (left_depth - right_depth).abs() > 1 {
                        return false;
                    }
                    depths.insert(Rc::as_ptr(&current), 1 + left_depth.max(right_depth));
                    drop(current_ref);
                    last = Some(current);
                    node = None;
                } else {
                    node = current_ref.right.clone();
                }
            }
        }

        true
    }
}