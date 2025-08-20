use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn run() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        assert_eq!(Rc::strong_count(&a), 1);
        if let Some(link) = a.tail() {
            let inner_rc = link.borrow();
            let list = &**inner_rc;
            assert!(matches!(*list, Nil));
        }

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        assert_eq!(Rc::strong_count(&a), 2);
        assert_eq!(Rc::strong_count(&b), 1);
        if let Some(link) = b.tail() {
            let inner_rc = link.borrow();
            // check if the two Rc pointers are same.
            assert!(Rc::ptr_eq(&inner_rc, &a));
        }

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        assert_eq!(Rc::strong_count(&b), 2);
        assert_eq!(Rc::strong_count(&a), 2);

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack.
        // println!("a next item = {:?}", a.tail());
    }
}
