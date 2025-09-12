// cargo run --bin 11-join-two-futures

use std::time::Duration;

fn main() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("{i} from fist task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let fut2 = async {
            for i in 1..5 {
                println!("{i} from second task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // ref: https://docs.rs/futures/latest/futures/future/fn.join.html
        //
        // Joins the result of two futures, waiting for them both to complete.
        //
        // This function will return a new future which await both futures to complete. The returned is tuple of both results.
        trpl::join(fut1, fut2).await;
    });
}
