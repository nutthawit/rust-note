use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        // use `lock` method to acquire the lock.
        let mut num = m.lock().unwrap();

        // use `Deref` to access data inside `MutexGuard`.
        //
        // Note: MutexGuard also implement `Drop` that releases the lock automatic
        // when goes out of scope.
        *num = 6;
    }
    println!("m = {m:?}");
}
