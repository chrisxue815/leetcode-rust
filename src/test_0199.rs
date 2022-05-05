use std::cell::RefCell;
use std::rc::Rc;

use crate::util::TreeNode;

struct Solution {}

// O(n) time. O(1) space.
impl Solution {
    pub fn right_side_view(root: Rc<RefCell<Option<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];

        if root.as_ref().borrow().is_some() {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root);

            while !queue.is_empty() {
                let back = queue.back().unwrap().as_ref().borrow().as_ref().unwrap().val;
                result.push(back);

                let level_len = queue.len();

                for _ in 0..level_len {
                    let curr = queue.pop_front().unwrap();
                    let curr = curr.as_ref().borrow();
                    let curr = curr.as_ref().unwrap();

                    if curr.left.as_ref().borrow().is_some() {
                        queue.push_back(Rc::clone(&curr.left));
                    }
                    if curr.right.as_ref().borrow().is_some() {
                        queue.push_back(Rc::clone(&curr.right));
                    }
                }
            }

            result
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::util;

    use super::*;

    #[test]
    fn it_works() {
        let test_data = util::load_test_json::<TestJson>(file!());

        for case in test_data.test_cases {
            let msg = serde_json::to_string(&case.args).unwrap();
            let root = TreeNode::from_array(case.args.root);
            let actual = Solution::right_side_view(root);
            assert_eq!(actual, case.expected, "{}", msg);
        }
    }

    #[derive(Serialize, Deserialize)]
    struct TestJson {
        test_cases: Vec<TestCase>,
    }

    #[derive(Serialize, Deserialize)]
    struct TestCase {
        args: Args,
        expected: Vec<i32>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    struct Args {
        root: Vec<Option<i32>>,
    }
}
