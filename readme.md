**Owned vs Unowned**

Owned values are data that a variable directly owns, meaning the variable is solely responsible for managing that data's memory.

Unowned values, in this context, refer to references (borrowed data) that provide temporary access to owned data without transferring ownership. 

> [Readmore]("https://www.integralist.co.uk/posts/rust-ownership/")

---

**static lifetime bound in function parameters**

It means the type does not contain any non-static references. Eg. the receiver can hold on to the type for as long as they want and it will never become invalid until they drop it. (*they and receiver means to function*)

It's important to understand this means that any owned data always passes a 'static lifetime bound, but a reference to that owned data generally does not.
[Readmore]("https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html#trait-bound")

```rust
use std::fmt::Debug;

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);
}
```

The compiler will tell you:

```bash
error[E0597]: `i` does not live long enough
  --> src/lib.rs:15:15
   |
15 |     print_it(&i);
   |     ---------^^--
   |     |         |
   |     |         borrowed value does not live long enough
   |     argument requires that `i` is borrowed for `'static`
16 | }
   | - `i` dropped here while still borrowed
```

---

**static lifetime bound on return type on closure**

what is `impl Fn(crate::surface::Action) -> Message + 'static`

```rust
pub fn applet_tooltip<'a, Message: 'static>(
        &self,
        content: impl Into<Element<'a, Message>>,
        tooltip: impl Into<Cow<'static, str>>,
        has_popup: bool,
        on_surface_action: impl Fn(crate::surface::Action) -> Message + 'static,
        parent_id: Option<window::Id>,
    )
```

`+ 'static`: The static lifetime bound on return type. It means the closure must not borrow any data from the local function's scope. All captured data must either be owned by the closure itself or have a 'static lifetime (meaning it lives for the entire duration of the program). This is a strong constraint that ensures the closure can be safely stored and used later without causing dangling pointers.

---

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

