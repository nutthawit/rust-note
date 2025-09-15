use std::sync::mpsc;
use std::thread;

fn main() {
    // mpsc::channel returns a tuple, the first is transmitter, and the second is receiver.
    // The abbreviations tx, rx are traditionally used in many fields for transmittr and receiver.
    let (tx, rx) = mpsc::channel();

    // The spawned thread needs to own the transmitter to be able to send messages.
    thread::spawn(move || {
        let val = String::from("hi");

        // The `send` method returns a `Result<T, E>`, os if the receiver
        // has been dropped and there's nowhere to send a value, the `send` will return an error.//
        // In this example, we're calling unwrap to panic. But in a real application
        // we would handle it property
        //
        // The `send` method takes ownership of its parameter, and
        // when value is moved, the receiver takes ownershipt of it.
        tx.send(val).unwrap();
    });

    // `recv`, which will block the main thread's and wait until a value is send down the channel.
    // Once a value is sent, `recv` will return `Result<T, E>`. When the transmitter closes, `recv` it's will return an error.
    let received = rx.recv().unwrap();
    println!("Got: {received}");

    // NOTE: The `try_recv` method doesn't block, but will return a `Result<T, E>` immediately:
    // an `Ok` value holding a message if one is available and an `Err` value if there aren't
    // any messages this time. *Using try_recv is useful if this thered has other work to do
    // while waiting for messages*
}
