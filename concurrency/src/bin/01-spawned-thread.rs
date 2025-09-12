use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 2) If we move `handle.join()` before main thread, the main thread will wait for the spawned thread to finish and then run for loop.
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // 1) In the first, the two threads continue alternating,until end of *main thread for loop*, the `handle.join()` tell main thread can't end until spawned thread is finished.

    handle.join().unwrap();
}
