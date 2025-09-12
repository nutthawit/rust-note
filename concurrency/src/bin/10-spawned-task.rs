// cargo run --bin 10-spawned-task

use std::time::Duration;

fn main() {
    trpl::run(async {
        // trpl::spawn_task == tokio::task::spawn
        //
        // Spawing a task enables the task to execute concurrently to other tasks. The spawned task may execute on
        // the current thread, or it may be sent to different thread to be executed. The specifics depend on the current
        // runtime configuration.
        //
        // The provided future will start running in the background immediately when `spawn` is called, even if you
        // don't await the returned `JoinHandle`.
        //
        // There is no guarantee that a spawned task will execute to completion. When a runtime is shutdown, all
        // outstanding task are dropped, regardless of the life cycle of that task.
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // Prevent a spawn task from being dropped after the runtime is shutdown.
        // behavior like the `std::thread::JoinHandle`.
        handle.await.unwrap();
    });
}
