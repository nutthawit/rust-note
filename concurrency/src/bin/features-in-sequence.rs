// use trpl::StreamExt;

// fn main() {
//     trpl::run(async {
//         let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//         let iter = values.iter().map(|n| n * 2);
//         let mut stream = trpl::stream_from_iter(iter);

//         while let Some(value) = stream.next().await {
//             println!("The value was: {value}");
//         }
//     });
// }
//

use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        // First time there are no messages yet in the channel, the main async task is paused,
        // It yields control back to the asynchronous runtime (trlp::run).
        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;
            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });

    // Represents an asynchronous stream of String
    ReceiverStream::new(rx)
}
