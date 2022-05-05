use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Rc<RefCell<Option<TreeNode>>>,
    pub right: Rc<RefCell<Option<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: Rc::new(RefCell::new(None)),
            right: Rc::new(RefCell::new(None)),
        }
    }

    pub fn from_array(vals: Vec<Option<i32>>) -> Rc<RefCell<Option<TreeNode>>> {
        let root = Rc::new(RefCell::new(None));
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Rc::clone(&root));

        for val in vals {
            let curr = queue.pop_front().unwrap();
            if let Some(val) = val {
                curr.as_ref().borrow_mut().replace(TreeNode::new(val));
                queue.push_back(Rc::clone(&curr.as_ref().borrow().as_ref().unwrap().left));
                queue.push_back(Rc::clone(&curr.as_ref().borrow().as_ref().unwrap().right));
            }
        }

        root
    }
}
