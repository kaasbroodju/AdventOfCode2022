use std::cell::RefCell;
use std::collections::{HashMap};
// use std::iter::{Enumerate, Map};
use std::rc::Rc;
use std::str::{FromStr};
// use std::time::Instant;

#[derive(PartialEq)]
struct TreeNode {
    pub value: usize,
    pub children: HashMap<String, Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn root() -> TreeNode {
        TreeNode {
            value: 0,
            children: HashMap::new(),
            parent: None
        }
    }

    fn new(x: Rc<RefCell<TreeNode>>) -> TreeNode {
        TreeNode {
            value: 0,
            children: HashMap::new(),
            parent: Some(x),
        }
    }

    fn calculate_amount_of_dir(&self, acc: &mut Vec<usize>) -> usize {

        let inside = self
            .children
            .values()
            .map(|dir| dir.borrow().calculate_amount_of_dir(acc))
            .sum::<usize>();

        acc.push(inside + self.value);

        inside + self.value
    }
}

#[inline]
pub fn first_part() -> usize {
    let root = Rc::new(RefCell::new(TreeNode::root()));
    let mut pwd = Rc::clone(&root);

    for line in include_str!("input.txt")
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .skip(1)
    {
        if line[0] == "$" {
            if line[1] == "cd" {
                if line[2] == ".." {
                    let current_clone = Rc::clone(&pwd);
                    pwd = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                } else {
                    let current_clone = Rc::clone(&pwd);
                    let map = &current_clone.borrow().children;
                    pwd = Rc::clone(map.get(line[2]).as_ref().unwrap());
                }
            }
        } else if line[0] == "dir" {
            {
                if pwd.borrow().children.get(line[1]).is_some() {println!("already have")}
            }
            let current_clone = Rc::clone(&pwd);
            let child = Rc::new(RefCell::new(TreeNode::new(current_clone)));
            pwd.borrow_mut().children.insert(String::from(line[1]), Rc::clone(&child));
        } else {
            pwd.borrow_mut().value += usize::from_str(line[0]).unwrap();
        }
    }

    let mut result: Vec<usize> = vec![];
    root.borrow().calculate_amount_of_dir(&mut result);

    result.iter().filter(|x| **x < (100000 as usize)).sum()
}

// previous attempts
// struct Dir {
//     parent: Option<*const Dir>,
//     children: HashMap<String, *const Dir>,
//     computed_size: usize,
// }

// struct Dir<'parent> {
//     parent: Option<&'parent mut Dir<'parent>>,
//     children: HashMap<&'parent str, &'parent mut Dir<'parent>>,
//     computed_size: usize,
// }
//
// impl Dir<'static> {
//     fn new<'a>(dir: &'a mut Dir<'a>) -> Dir<'a> {
//         Dir {
//             parent: Some(dir),
//             children: Default::default(),
//             computed_size: 0
//         }
//     }
//
//     fn root<'a>() -> Dir<'a> {
//         Dir {
//             parent: None,
//             children: Default::default(),
//             computed_size: 0
//         }
//     }
//
//     fn go_up<'parent>(&mut self) -> &mut Option<&'static mut Dir<'static>> {
//         self.parent.borrow_mut()
//     }
//
//     fn go_down<'parent>(&'parent mut self, key: &'parent str) -> &mut Dir<'parent> {
//         *self.children.get_mut(
//             key
//         ).expect("dir not found").borrow_mut()
//     }
//
//     fn create_new_dir(&mut self, key: &str) {
//         self.children.insert(key, &mut Dir::new(self));
//     }
//
//     fn add_file(&mut self, size: usize) {
//         self.computed_size += size;
//     }
// }

#[inline]
pub fn second_part() -> usize {
    let root = Rc::new(RefCell::new(TreeNode::root()));
    let mut pwd = Rc::clone(&root);

    for line in include_str!("input.txt")
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .skip(1)
    {
        if line[0] == "$" {
            if line[1] == "cd" {
                if line[2] == ".." {
                    let current_clone = Rc::clone(&pwd);
                    pwd = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                } else {
                    let current_clone = Rc::clone(&pwd);
                    let map = &current_clone.borrow().children;
                    pwd = Rc::clone(map.get(line[2]).as_ref().unwrap());
                }
            }
        } else if line[0] == "dir" {
            let current_clone = Rc::clone(&pwd);
            let child = Rc::new(RefCell::new(TreeNode::new(current_clone)));
            pwd.borrow_mut().children.insert(String::from(line[1]), Rc::clone(&child));
        } else {
            pwd.borrow_mut().value += usize::from_str(line[0]).unwrap();
        }
    }

    let mut result: Vec<usize> = vec![];
    root.borrow().calculate_amount_of_dir(&mut result);
    const TARGET_THRESHOLD: usize = 30000000;
    const AVAILABLE_SPACE: usize = 70000000;
    let threshold = AVAILABLE_SPACE - result.iter().max().unwrap();

    *result.iter().filter(|x| threshold + *x > TARGET_THRESHOLD).min().unwrap()
}