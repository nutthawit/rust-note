use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use crate::rct::List::{Cons, Nil};
    use std::rc::Rc;

    #[test]
    fn run() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        assert_eq!(1, Rc::strong_count(&a));

        let b = Cons(3, Rc::clone(&a));
        assert_eq!(2, Rc::strong_count(&a));

        {
            let c = Cons(4, Rc::clone(&a));
            assert_eq!(3, Rc::strong_count(&a));
        }
        // count after c gone out of scope.
        assert_eq!(2, Rc::strong_count(&a));
    }
}
