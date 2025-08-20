use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::Node;
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[test]
    fn run() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        let parent = leaf.parent.borrow().upgrade();
        assert!(parent.is_none());

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        let parent = leaf.parent.borrow().upgrade();
        assert!(parent.is_some());
    }
}
