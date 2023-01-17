use std::rc::{Weak, Rc};
use std::cell::RefCell;


pub struct Node<T> {
    data: T,
    children: Vec<Rc<RefCell<Node<T>>>>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node::new_with_parent(data, None)
    }

    pub fn new_with_parent(data: T, parent: Option<Weak<RefCell<Node<T>>>>) -> Node<T> {
        return Node{
            data: data,
            children: vec![],
            parent: None,
        }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node<T>>>) {
        self.children.push(Rc::clone(&child));
    }
}

pub struct Tree<T> {
    root: Rc<RefCell<Node<T>>>,
    current: Rc<RefCell<Node<T>>>,
}

impl<T> Tree<T> {
    pub fn new(data: T) -> Tree<T>{
        let root = Rc::new(RefCell::new(Node::new(data)));
        Tree {
            root: Rc::clone(&root),
            current: Rc::clone(&root),
        }
    }

    pub fn add_child(&self, data: T) {
        let child =  Rc::new(
            RefCell::new(
                Node::new_with_parent(
                    data, 
                    Some(Rc::downgrade(&Rc::clone(&self.current)))
                )
            )
        );
        self.current.borrow_mut().add_child(child);
    }

    pub fn goto_child(&mut self, i: usize) {
        self.current = Rc::clone(&Rc::clone(&self.current).borrow().children[i]);
    }

    pub fn goto_parent(&mut self) {
        match &Rc::clone(&self.current).borrow().parent {
            Some(opt_reference) => {
                match Weak::upgrade(opt_reference) {
                    Some(reference) => {
                        self.current = reference;
                    },
                    None => {panic!("parent node was deleted before child")}
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
            tree.add_child(i);
            println!("{}", tree.current.borrow().data);
            tree.goto_child(0);
        }
        tree.goto_parent();
        for i in 1..4 {
            tree.goto_parent();
        }
    }
}