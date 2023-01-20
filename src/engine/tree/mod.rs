use std::cell::RefCell;
use std::rc::{Rc, Weak};
use core::slice::Iter;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    children: Vec<Rc<RefCell<Node<T>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node::new_with_parent(data, None)
    }

    pub fn new_with_parent(data: T, parent: Option<Weak<RefCell<Node<T>>>>) -> Node<T> {
        return Node {
            data: data,
            children: vec![],
            parent: parent,
        };
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node<T>>>) {
        self.children.push(Rc::clone(&child));
    }
}

#[derive(Debug)]
pub struct Tree<T> {
    pub root: Rc<RefCell<Node<T>>>,
    pub current: Rc<RefCell<Node<T>>>,
}

impl<T> Tree<T> {
    pub fn new(data: T) -> Tree<T> {
        let root = Rc::new(RefCell::new(Node::new(data)));
        Tree {
            root: Rc::clone(&root),
            current: Rc::clone(&root),
        }
    }

    pub fn add_child(&self, data: T) {
        let child = Rc::new(RefCell::new(Node::new_with_parent(
            data,
            Some(Rc::downgrade(&Rc::clone(&self.current))),
        )));
        self.current.borrow_mut().add_child(child);
    }

    pub fn has_no_child(&self) -> bool {
        self.current.borrow().children.is_empty()
    }

    pub fn has_children(&self) -> bool {
        !self.has_no_child()
    }

    pub fn has_parent(&self) -> bool {
        self.current.borrow().parent.is_some()
    }

    pub fn goto_child(&mut self, i: usize) {
        self.current = Rc::clone(&Rc::clone(&self.current).borrow().children[i]);
    }

    pub fn goto_last_child(&mut self) {
        let last_child_i = self.current.borrow().children.len() - 1;
        self.current = Rc::clone(&Rc::clone(&self.current).borrow().children[last_child_i]);
    }

    pub fn goto_parent(&mut self) {
        match &Rc::clone(&self.current).borrow().parent {
            Some(opt_reference) => match Weak::upgrade(opt_reference) {
                Some(reference) => {
                    self.current = reference;
                }
                None => {
                    panic!("parent node was deleted before child")
                }
            },
            None => {
                panic!("no parent to go to")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_children_and_going_back() {
        let mut tree = Tree::<u8>::new(0);
        for i in 1..5 {
            let child_val = i + tree.current.borrow().data;
            tree.add_child(i + child_val);
            println!("data of current: {}", tree.current.borrow().data);
            println!("parent of current: {:?}", tree.current.borrow().parent);
            tree.goto_child(0);
        }
        for _ in 1..5 {
            println!("data of current: {}", tree.current.borrow().data);
            tree.goto_parent();
        }
    }

    #[test]
    fn test_adding_children_and_going_back_2() {
        let mut tree = Tree::<u8>::new(0);
        for i in 1..5 {
            let child_val = i + tree.current.borrow().data;
            tree.add_child(i + child_val);
            println!("data of current: {}", tree.current.borrow().data);
            println!("parent of current: {:?}", tree.current.borrow().parent);
            tree.goto_last_child();
        }
        for _ in 1..5 {
            println!("data of current: {}", tree.current.borrow().data);
            tree.goto_parent();
        }
    }
}
