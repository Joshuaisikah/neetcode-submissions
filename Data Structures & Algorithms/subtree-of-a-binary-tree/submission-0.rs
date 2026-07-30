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
    fn serialize(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            None => "$#".to_string(),
            Some(node) => {
                let node = node.borrow();
                format!(
                    "${}{}{}",
                    node.val,
                    Self::serialize(&node.left),
                    Self::serialize(&node.right)
                )
            }
        }
    }

    fn z_function(s: &[u8]) -> Vec<usize> {
        let n = s.len();
        let mut z = vec![0usize; n];
        let (mut l, mut r) = (0usize, 0usize);

        for i in 1..n {
            if i <= r {
                z[i] = (r - i + 1).min(z[i - l]);
            }
            while i + z[i] < n && s[z[i]] == s[i + z[i]] {
                z[i] += 1;
            }
            if i + z[i] > 0 && i + z[i] - 1 > r {
                l = i;
                r = i + z[i] - 1;
            }
        }
        z
    }

    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let serialized_root = Self::serialize(&root);
        let serialized_sub_root = Self::serialize(&sub_root);
        let combined = format!("{}|{}", serialized_sub_root, serialized_root);

        let z_values = Self::z_function(combined.as_bytes());
        let sub_len = serialized_sub_root.len();

        for i in (sub_len + 1)..combined.len() {
            if z_values[i] == sub_len {
                return true;
            }
        }
        false
    }
}