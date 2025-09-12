// cargo run --bin 15-stream-with-timeout
use std::pin::pin;
use std::time::Duration;
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        // .timeout == tokio::StreamExt.timeout
        // Applies a per-item timeout to the passed stream.
        //
        // timeout() takes a Duration that represents the maximum amount of time each element of the stream has to complete before timing out.
        //
        // ref: https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.timeout
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(m) => println!("{m}"),
                Err(e) => eprintln!("Problem: {e:?}"),
            }
        }

        /* OUTPUT
        Message: 'a 0'
        Problem: Elapsed(())
        Message: 'b 1'
        Message: 'c 2'
        Problem: Elapsed(())
        Message: 'd 3'
        Message: 'e 4'
        Problem: Elapsed(())
        Message: 'f 5'
        Message: 'g 6'
        Problem: Elapsed(())
        Message: 'h 7'
        Message: 'i 8'
        Problem: Elapsed(())
        Message: 'j 9'

        Between every other pair of messages, a Problem: Elapsed(()) error.

        The timeout doesn’t prevent the messages from arriving in the end. We still get all of the original messages, because our channel is unbounded: it can hold as many messages as we can fit in memory. If the message doesn’t arrive before the timeout, our stream handler will account for that, but when it polls the stream again, the message may now have arrived. */
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (i, m) in messages.into_iter().enumerate() {
            // set sleep time 100ms for even numbers and 300ms for odd numbers
            let st = if i % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(st)).await;

            tx.send(format!("Message: '{m} {i}'")).unwrap();
        }
    });

    // Convert the `rx` receiver from the `trpl::channel` into a `Stream` cause we can call the `next` method on it.
    ReceiverStream::new(rx)
}
