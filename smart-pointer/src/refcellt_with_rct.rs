use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn run() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        if let Cons(b_value, b_tail) = &b {
            assert_eq!(*b_value.borrow(), 3);
            if let Cons(a_value, _) = &**b_tail {
                assert_eq!(*a_value.borrow(), 15);
            }
        }

        if let Cons(c_value, c_tail) = &c {
            assert_eq!(*c_value.borrow(), 4);
            if let Cons(a_value, _) = &**c_tail {
                assert_eq!(*a_value.borrow(), 15);
            }
        }
    }
}
