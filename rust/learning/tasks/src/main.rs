use std::comm::TryRecvError;

fn main() {
    println!("Hello, world!");
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    spawn(move || {
        tx1.send("Hello from a task!".to_string());
        let message: Result<String, TryRecvError> = rx2.try_recv();
        //let message = rx2.recv();

        match message {
            Result::Ok(response)    => { println!("Child received a message: {}", response); },
            Result::Err(error)      => {
                match error {
                    TryRecvError::Empty => { println!("Channel is empty."); },
                    TryRecvError::Disconnected => { println!("Channel disconnected."); },
                }
            },
        }
    });

    let message = rx1.recv();
    println!("Parent received a message: {}", message);

    tx2.send("main says hello!".to_string());
}
