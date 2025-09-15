// cargo run --bin 01-spawned-thread

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1)); // The calls `thread::sleep` force a thread to stop its execution for a short duration, allowing a different thread to run.
        }
    });

    // 2) If we move `handle.join()` before main thread, the main thread will wait for the spawned thread to finish and then run their for loop.
    //
    // try uncomment this:
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // 1) In the first, the two threads continue alternating,until end of *main thread for loop*, the `handle.join()` tell main thread can't end until spawned thread is finished.
    //
    // Note: If we don't call handle.join(), the spawned thread will be shutdown immediately, when main thread finish.

    handle.join().unwrap();
}
