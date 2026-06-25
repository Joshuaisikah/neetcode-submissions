// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

struct Codec;

impl Codec {
    fn new() -> Self {
        Codec
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "N".to_string();
        }
        let mut res = Vec::new();
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(root);

        while let Some(node_opt) = queue.pop_front() {
            match node_opt {
                None => res.push("N".to_string()),
                Some(node) => {
                    let n = node.borrow();
                    res.push(n.val.to_string());
                    queue.push_back(n.left.clone());
                    queue.push_back(n.right.clone());
                }
            }
        }
        res.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let vals: Vec<&str> = data.split(',').collect();
        if vals[0] == "N" {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode {
            val: vals[0].parse().unwrap(),
            left: None,
            right: None,
        }));
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root.clone());
        let mut index = 1;

        while let Some(node) = queue.pop_front() {
            if vals[index] != "N" {
                let left = Rc::new(RefCell::new(TreeNode {
                    val: vals[index].parse().unwrap(),
                    left: None,
                    right: None,
                }));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            index += 1;
            if vals[index] != "N" {
                let right = Rc::new(RefCell::new(TreeNode {
                    val: vals[index].parse().unwrap(),
                    left: None,
                    right: None,
                }));
                node.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
            index += 1;
        }
        Some(root)
    }
}