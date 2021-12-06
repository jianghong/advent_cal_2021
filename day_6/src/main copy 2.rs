use std::sync::mpsc::sync_channel;
use std::{thread, time};

fn main() {
    let (tx, rx) = sync_channel(3);


    for i in 0..5 {
        // It would be the same without thread and clone here
        // since there will still be one `tx` left.
        let tx = tx.clone();
        // cloned tx dropped within thread
        thread::spawn(move || {
            let ten_millis = time::Duration::from_millis(100);
            if i == 2 {
                thread::sleep(ten_millis * 1);
            } else if i == 1{
                thread::sleep(ten_millis * 5);
            } else {

                thread::sleep(ten_millis * 10);
            }
            tx.send(format!("{}", i)).unwrap()
        });
    }

    // Drop the last sender to stop `rx` waiting for message.
    // The program will not complete if we comment this out.
    // **All** `tx` needs to be dropped for `rx` to have `Err`.
    drop(tx);

    // Unbounded receiver waiting for all senders to complete.
    while let Ok(msg) = rx.recv() {
        println!("{}", msg);
    }

    println!("completed");
}