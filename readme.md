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

---

The `Sync` trait indicates that it is safe to be referenced from multiple threads.

The `Send` trait indicates that ownership can be transfered between threads.

Almost all primitive types are `Send`.

All primitive types implement `Sync`.

Any types composed entirely of `Send` is automatically marked as `Send`.

Any types composed entirely of `Sync` is also implement `Sync`.

***<span style="color:red">Manullay implementing these traits is unsafe</span>***

## Concurrency (Thread and Asynchronous)

***asynchronous programming*** is where operations may not finish sequentially in the order they were started.

Illustrate of *concurrency*, you have two different projects checked out on your computer, and when you get bored or stuck on one project, you switch to the other. You're just one person, so you can't make progress on both tasks at the exact same time, but you can multi-task, making progress on one at a time by switching between them.

```
        |___________________________________________|
Man --> | --> A1 -   --> A2   --> A3 --> A4         | Task A
        |         \ /      \ /            \         |
        |         B1       B2             B3 --> B4 | Task B
        |___________________________________________|
```

*feature* is a value that may not be ready now but will become ready at some point in future, In Rust feature are type that implement the `Feature` trait, Each future hold its own information about the progress that has been made and what "ready" means.

> The futures crate is an official home for Rust experimentation for async code, and it’s actually where the Future trait was originally designed. 

**futures in Rust are lazy: they don’t do anything until you ask them to with the `await` keyword.**

> Note: This is different from the behavior we saw in the thread::spawn, where the closure we passed to another thread started running immediately.

---

[look at full code here before continue.](concurrency/src/bin/80-basic-async-program.rs)

When Rust sees a block marked with the `async` keyword, it compiles it into a unique, anonymous data type that implements the `Future` trait. When Rust sees a function marked with `async`, it compiles it into a non-async function whose body is an async block. An async function’s return type is the type of the anonymous data type the compiler creates for that async block.

Thus, writing `async fn` is equivalent to writing a function that returns a future of the return type. To the compiler, a function definition such as the `async fn page_title` is equivalent to a non-async function defined like this:

```rust
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

Let’s walk through each part of the transformed version:

1. It uses the `impl Trait` syntax.

2. The returned trait is a `Future` with an associated type of `Output`. Notice that the Output type is `Option<String>`, which is the same as the original return type from the async fn version of `page_title`.

3. All of the code called in the body of the original function is wrapped in an `async move` block. Remember that blocks are expressions. This whole block is the expression returned from the function.

4. This async block produces a value with the type `Option<String>`, as just described. That value matches the `Output` type in the return type. This is just like other blocks you have seen.

5. The new function body is an `async move` block because of how it uses the `url` parameter.

---

### What is `await`

The only place we can use the `await` keyword is in *async functions* or *blocks*, and Rust won’t let us mark the special main function as async.

> Note: The reason main can’t be marked async is that async code needs a runtime: a Rust crate that manages the details of executing asynchronous code. A program’s main function can initialize a runtime, but it’s not a runtime itself.

Each *await* point—that (every place where the code uses the `await` keyword—represents) is a place where control is handed back to the runtime (Rust will pauses *async blocks* and hands control back to a runtime) Everything between await points is synchronous. That means if you do a bunch of work in an async block without an await point, that `Future` will block any other `Future`s from making progress.

### Primary situations where you must use .await

1. **Waiting for an Asynchronous operation to complete:** This is the most common use case. Any function that returns a `Future` needs to be awaited to get its result.

```rust
// Awaiting a web request to finish
let resp = reqwest::get("https://example.com").await?;
let body = response.text().await?;

// Awaiting a file read operation
let contents = tokio::fs::read_to_string("my_file.txt").await?;
```

2. **Waiting for a Channel to receive a message:** Receiving a message is an asynchronous operation. You must `.await` the receive call to wait for a message to be sent.

```rust
// Awaiting the next message from a channel
let message = rx.recv().await?;
```

3. **Waiting for a Stream to produce an item:** Iterating over a `Stream` requires `.await`. The stream may not have an item ready immediately, so the `next()` method returns a `Future` that you must await.

```rust
// Awaiting the next item from an async stream
while let Some(item) = stream.next().await {
    // ... process item
}
```

4. **Waiting for a Time or a Delay:** When you need to pause execution fro a specific duration without blocking the entrie thread, you use an asynchronous sleep function and `.await` it.

```rust
// Awaiting an asynchronous sleep
tokio::time::sleep(Duration::from_secs(1)).await
```

**Summary** Think of `.await` as a signal to the async runtime: "I can't continue until this operation is done." Please pause me and go work on other tasks in the meantime.

**Let's look more on how they are used**

- [`Feture` race to indicate which one will finish first](concurrency/src/bin/09-future-race.rs)
- [Create a new async task with `tokio::task::spawn`](concurrency/src/bin/10-spawned-task.rs)
- [Join two `Futures` to make a concurrency work](concurrency/src/bin/11-join-two-futures.rs)

### `tokio::task::yield_now`

When you want to hands control back to a runtime immediately, for avoid blocking for too long processing. [example code](concurrency/src/bin/99-hands-control-back-to-runtime-immediately.rs)

### Stream

Stream is like Iterator but for asynchronous.

*async channel receiver* also stream, The first difference is time: iterators are synchronous, while the channel receiver is asynchronous The second is the API. When working directly with Iterator, we call its synchronous `next` method. With the `trpl::Receiver` stream in particular, we called an asynchronous `recv` method instead. Otherwise, these APIs feel very similar.

[example of async channel receiver](concurrency/src/bin/12-message-passing-between-futures.rs)

[building a little stream of messages](concurrency/src/bin/14-basic-stream.rs)

[add timeout to stream](concurrency/src/bin/15-stream-with-timeout.rs)

[merge streams](concurrency/src/bin/16-merged-streams.rs)

### How `Future` trait work

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

```

`Output` is what the future resolves to.

`Future` has `poll` method, which take a:
1. `Pin` reference for its `self` param
2. a mutable reference to a `Context` type
3. method returns type is `Poll`

```rust
enum Poll<T> {
    Ready(T),
    Pending,
}
```

`Pending` variant indicates that the future still has work to do, so the caller will need to check again later.

`Ready` variant indicates that the future has finished its work and the `T` value is available.

When you see code that uses `await`, under the hood its calls `poll`, look back at [fn page_title](concurrency/src/bin/08-basic-async-program.rs), Rust compiles it into something like this:

```rust
let mut page_title_fut = page_title(url);
loop {
    match page_title_fut.poll() {
        Ready(page_title) => match page_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
        Pending => {
            // continue
        }
    }   
}
```

Loop is for when the future is still `Pending`. We need some way to try again, until the future is finally ready.

Inside the `Pending` arm. Rust makes sure that the loop can hand off control to *something that can pause work on this future, to work on other futures, and then check this one again later*. That something is an *async runtime*, and this *scheduling and coordination work* is one of its main jobs.

Look back at [example code](concurrency/src/bin/12-message-passing-between-futures.rs) on `rx.recv`. The `recv` call returns a future, and awaiting the future polls it. We noted that a runtime will pause the future until it's ready with either `Some(message)` or `None` when the channel closes. The runtime knows the future isn't ready when it returns `Poll::Pending`. Conversely, the runtime knows the future is ready and advances it when `poll` return `Poll::Ready(Some(message))` or `Poll::Ready(None)`

### Pin and Unpin trait

ส่วนนี้จะอธิบายว่าเทรต Pin และ Unpin ถูกนำมาใช้ในภาษา Rust อย่างไร โดยเฉพาะอย่างยิ่งในบริบทของการเขียนโปรแกรมแบบอะซิงโครนัส (asynchronous) และเทรต Future บทความนี้อธิบายว่า Future จำเป็นต้องใช้ตัวชี้แบบ Pin-wrapped (ตัวชี้ที่ถูกห่อหุ้มด้วย Pin) เพื่อให้สามารถถูก "poll" ได้ Pin ถูกอธิบายว่าเป็น wrapper (ตัวห่อหุ้ม) สำหรับชนิดข้อมูลที่คล้ายกับตัวชี้ ซึ่งบังคับใช้ข้อจำกัดในการใช้งานตัวชี้ แต่ตัวมันเองไม่ใช่ตัวชี้

บทความยังอธิบายว่า await จะทำการ pin future โดยอัตโนมัติ (implicitly pins) แต่เมื่อมีการย้าย futures ไปยัง collection เช่น Vec เพื่อส่งไปยังฟังก์ชันอย่าง join_all นั้น futures จะต้องถูก pin อย่างชัดเจน (explicitly pinned) เหตุผลก็คือบล็อก async สามารถสร้างชนิดข้อมูลที่มีการอ้างอิงถึงตัวเองได้ (self-referential types) และการย้ายพวกมันอาจไม่ปลอดภัย Unpin ถูกแนะนำให้เป็นเทรตชนิด marker (เครื่องหมาย) ที่บอกคอมไพเลอร์ว่าชนิดข้อมูลนั้นปลอดภัยที่จะย้ายได้ ชนิดข้อมูลส่วนใหญ่ใน Rust จะใช้ Unpin โดยอัตโนมัติ แต่ futures ที่มีการอ้างอิงถึงตัวเองจะไม่ได้ใช้ ส่วนสุดท้ายของบทความสรุปว่า Pin และ Unpin มีความสำคัญส่วนใหญ่สำหรับการสร้างไลบรารีหรือรันไทม์ระดับล่าง และการทำความเข้าใจเทรตเหล่านี้จะช่วยในการแก้ไขข้อความแสดงข้อผิดพลาดที่เกี่ยวข้องกับการ pinning ได้

##### โครงสร้างข้อมูลแบบ Self-Referential และเหตุผลที่ไม่ควรย้ายมัน

ข้อมูลแบบ self-referential (การอ้างอิงถึงตัวเอง) คือโครงสร้างข้อมูลที่เก็บตัวชี้ (pointer) หรือการอ้างอิง (reference) ไปยังตัวมันเองหรือไปยังส่วนอื่น ๆ ภายในตัวมัน ตัวอย่างที่ง่ายที่สุดคือโหนดของลิงก์ลิสต์ (linked list) ที่มีตัวชี้ไปยังโหนดถัดไป ซึ่งอาจมีตัวชี้ย้อนกลับไปที่โหนดก่อนหน้าได้ด้วย ตัวอย่างที่เกี่ยวข้องในบริบทของ async ในภาษา Rust คือ Future ที่ต้องจัดเก็บอ็อบเจกต์ชั่วคราวซึ่งถือการอ้างอิงไปยังข้อมูลภายในตัว Future นั้นเอง

##### ทำไมการย้ายถึงไม่ปลอดภัย

การย้ายโครงสร้างข้อมูลที่มีการอ้างอิงถึงตัวเองในหน่วยความจำสามารถนำไปสู่ปัญหา dangling pointers (ตัวชี้ที่ชี้ไปยังตำแหน่งหน่วยความจำที่ถูกปลดไปแล้ว) หรือ invalid references (การอ้างอิงที่ไม่ถูกต้อง) ซึ่งถือเป็นเรื่องไม่ปลอดภัยและอาจทำให้เกิดพฤติกรรมที่ไม่คาดคิดได้ เมื่อตัวแปรถูกย้าย ข้อมูลของมันจะถูกคัดลอกไปยังตำแหน่งใหม่ในหน่วยความจำ และหน่วยความจำเดิมจะถูกปลดปล่อย ปัญหาหลักของข้อมูลแบบ self-referential คือตัวชี้ภายในโครงสร้างยังคงชี้ไปยังตำแหน่งหน่วยความจำ เดิม ไม่ใช่ตำแหน่งใหม่

ลองนึกถึงภาพจำลองนี้:

สมมติว่าคุณมีแผนที่บ้านที่แต่ละห้องเป็นกระดาษหนึ่งแผ่น และคุณมีโน้ตในแผ่นกระดาษห้องนั่งเล่นที่เขียนว่า "ห้องครัวอยู่ห่างออกไป 3 เมตรทางขวาของฉัน" โน้ตนี้คือการอ้างอิงถึงตัวเอง เพราะมันอธิบายตำแหน่งโดยอ้างอิงจากตัวห้องนั่งเล่นเอง

ถ้าคุณย้ายกระดาษแผ่นห้องนั่งเล่นไปยังตำแหน่งใหม่ โน้ตนั้นก็ยังคงเขียนว่า "3 เมตรทางขวาของฉัน" แต่ตอนนี้การอ้างอิงนั้นไม่ถูกต้องแล้ว เพราะกระดาษแผ่นห้องครัวยังคงอยู่ที่ตำแหน่งเดิม การอ้างอิงนั้นจึงกลายเป็นไม่ถูกต้อง

ในทำนองเดียวกัน เมื่อโครงสร้างข้อมูลแบบ self-referential ถูกย้าย ตัวชี้ภายในของมันจะกลายเป็นไม่ถูกต้อง เพราะพวกมันยังคงชี้ไปยังที่อยู่หน่วยความจำเก่าซึ่งไม่ถูกต้องอีกต่อไป นี่คือเหตุผลที่คอมไพเลอร์ของ Rust ป้องกันไม่ให้ Future ที่มีการอ้างอิงถึงตัวเองถูกย้าย เว้นแต่จะถูก pin ซึ่งเป็นกระบวนการที่รับประกันว่าที่อยู่หน่วยความจำของมันจะไม่เปลี่ยนแปลง

---

A task is similar to a thread, but instead of being managed by the operating system, it’s managed by library-level code: the runtime.


