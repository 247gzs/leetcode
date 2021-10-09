// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }



// 方法一
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if root.is_none() {
            return ans;
        }

        let mut stack = vec![root];
        while !stack.is_empty() {
            if let Some(x) = stack.pop().flatten() {
                ans.push(x.borrow().val);
                stack.push(x.borrow().right.clone());
                stack.push(x.borrow().left.clone());
            }
        }
        ans
    }
}


// 方法二
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(x) = node {
                result.push(x.borrow().val);
                dfs(&x.borrow().left, result);
                dfs(&x.borrow().right, result);
            }
        }

        let mut ans = vec![];
        dfs(&root, &mut ans);
        ans
    }
}
