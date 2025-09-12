// cargo run --bin 12-message-passing-between-futures
use std::pin::Pin;
use std::time::Duration;

fn main() {
    trpl::run(async {
        // trpl::channel() == tokio::sync::mpsc::unbounded_channel()
        //
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        // Move tx into async block, it would be dropped once that block ends, and make channel close respective.
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            // rx.recv() == tokio::sync::mpsc::UnboundedReceiver.recv()
            //
            // The channel is closed when all senders have been dropped, or when close is called.
            //
            // If there are no messages in the channel’s buffer, but the channel has not yet been closed, this method will sleep until a message is sent or the channel is closed.
            //
            // This method returns `None` if the channel has been closed and there are no remaining messages in the channel’s buffer. This indicates that no further values can ever be received from this Receiver.
            while let Some(value) = rx.recv().await {
                println!("Got: {value}");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];

        // To make the messages arrive at half-second intervals, rather than all in a rust after 2 seconds. We need `join` for execute each of async block separately.
        trpl::join_all(futures).await;
    });
}
