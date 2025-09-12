use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let mut messages = get_messages();

        // Then we use a `while let` loop to print all the messages from the stream.
        while let Some(m) = messages.next().await {
            println!("{m}");
        }

        /* OUTPUT
        Message: 'a'
        Message: 'b'
        Message: 'c'
        Message: 'd'
        Message: 'e'
        Message: 'f'
        Message: 'g'
        Message: 'h'
        Message: 'i'
        Message: 'j' */

        /* Note: behavior in this code we can do this with the regular Receiver API or even the regular Iterator API */
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for m in messages {
        tx.send(format!("Messsage: '{m}'")).unwrap();
    }

    // Convert the `rx` receiver from the `trpl::channel` into a `Stream` cause we can call the `next` method on it.
    ReceiverStream::new(rx)
}
