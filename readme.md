`Box<T>` allow you to store data on the heap rather than the stack.

What remains on the stack is the pointer to the heap.

The `Box<T>` is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values (T value) to be treated like refrences.

When `Box<T>` value goes out of scope, the heap data that the box pointing to is cleaned up because of the `Drop` trait implementation.

---

You can enable multiple ownership by using `Rc<T>`.

The `Rc<T>` type keeps track of number of references to a value to determine whether or not the value still in use.

Invoking *clone* on Rc produces a new pointer to the same allocation in the heap.

If there zero reference to a value, the value can be cleaned up.

Via the immutable refrences, `Rc<T>` allows you to share data between multiple parts of your program for reading only.

If `Rc<T>` allowed you to have multiple mutable reference too, you might violate one of the borrowing rules, multiple mutable borrows to the same place can cause
data races.

---

With `RefCell<T>`, borrowing rules are enforced at runtime. 

`Rc<T>`, `RefCell<T>` is only for use in single-threaded.

The reasons to choose Box<T>, Rc<T>, or RefCell<T>:

  - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
  - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
  - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

With `RefCell<T>`, we use the *borrow* and *borrow_mut* methods, which are part of the safe API that belongs to `RefCell<T>`. The *borrow* method returns the smart pointer type `Ref<T>`, and *borrow_mut* returns the smart pointer type `RefMut<T>`. Both types implement `Deref`, so we can treat them like regular references.

The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. Every time we call *borrow*, the `RefCell<T>` increases its count of how many immutable borrows are active. When a `Ref<T>` value goes out of scope, the count of immutable borrows goes down by 1. Just like the compile-time borrowing rules, **`RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.**

