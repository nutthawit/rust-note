// cargo run --bin 16-merged-streams
use std::pin::pin;
use std::time::Duration;
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));

        // throttle: Slows down a stream by enforcing a delay between items.
        //
        // ref: https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.throttle
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));

        // merge: Combine two streams into one by interleaving the output of both as it is produced.
        //
        // Values are produced from the merged stream in the order they arrive from the two source streams. If both source streams provide values simultaneously, the merge stream alternates between them.
        //
        // The merged stream completes once both source streams complete.
        let merged = messages.merge(intervals).take(20);

        let mut stream = pin!(merged);
        while let Some(result) = stream.next().await {
            match result {
                Ok(m) => println!("{m}"),
                Err(e) => eprintln!("Problem: {e:?}"),
            }
        }

        /* OUTPUT
        Interval: 1
        Message: 'a', Index: '0'
        Interval: 2
        Interval: 3
        Problem: Elapsed(())
        Interval: 4
        Message: 'b', Index: '1'
        Interval: 5
        Message: 'c', Index: '2'
        Interval: 6
        Interval: 7
        Problem: Elapsed(())
        Interval: 8
        Message: 'd', Index: '3'
        Interval: 9
        Message: 'e', Index: '4'
        Interval: 10
        Interval: 11
        Problem: Elapsed(())
        Interval: 12 */
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (i, m) in messages.into_iter().enumerate() {
            let st = if i % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(st)).await;

            if let Err(e) = tx.send(format!("Message: '{m}', Index: '{i}'")) {
                eprintln!("Cannot send message '{m}': {e}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(e) = tx.send(count) {
                eprintln!("Could not send interval {count}: {e}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}
