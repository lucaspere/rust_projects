
# Day 15 - Channels
Rust implements the CSP (communication sequential process) model in the ``std::sync::mpsc`` library. Channel allows us to communicate between threads by passing message each other via channel. For example:

```rs
let (sender, receiver) = mpsc::channel();

    let task_sender = thread::spawn(move || {
        for task in 0..10 {
            let sender = sender.clone();
            thread::spawn(move || {
                sender.send(task).unwrap();
            });
            sleep(std::time::Duration::from_millis(500));
        }
    });

    task_sender.join().unwrap();
    loop {
        match receiver.recv() {
            Ok(msg) => println!("Message received: {:#?}", msg),
            
            Err(_) => {
                println!("Channel closed!");
                break;
            }
        }
    }
```

## Channel Types
### Bounded
Channel with limit numbers of messages is called bounded. This type of channel is usefull when we want to limit the number of receivers and safe some memory usage. For example, we can create a bounded channel for TcpSockets to limit only 32 connections. With bounded channel it is important to handle the backpressure.

### Unbounded
This is the pattern of core mpsc, this not have limit to connections sended, so if not handle accordyling, it can fill up all available memory and cause the system crash.

## Ownership and Borrowing
Channel take ownership of the data it is sended, because with references the receiver can receive a data that is already dropped by the sender scope.

## Error Handling
The channel receiver returns a ``Err`` to indicate that no more data will be sent. However, it's not means that there isn't data available to parse, since the all the data is buffered. The channel sender returns a ``Err`` when the data to send won't be received by the receiver.


## Tokio Channels

```rs
async fn tokio_channels() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    let tx = tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });

    let rx = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Message received: {:#?}", msg);
        }
    });

    select! {
        _ = tx => println!("Sender task completed!"),
        _ = rx => println!("Receiver task completed!"),
    }
}
```

## Code
```rs
use std::{
    sync::mpsc,
    thread::{self, sleep},
};

use tokio::select;

pub fn comm_channel() {
    let (sender, receiver) = mpsc::channel();

    let task_sender = thread::spawn(move || {
        for task in 0..10 {
            let sender = sender.clone();
            thread::spawn(move || {
                if task == 5 {
                    drop(sender);
                } else {
                    sender.send(task).unwrap();
                }
            });
            sleep(std::time::Duration::from_millis(500));
        }
    });

    task_sender.join().unwrap();
    loop {
        match receiver.recv() {
            Ok(msg) => println!("Message received: {:#?}", msg),
            Err(_) => {
                println!("Channel closed!");
                break;
            }
        }
    }
}

async fn tokio_channels() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    let tx = tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });

    let rx = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("Message received: {:#?}", msg);
        }
    });

    select! {
        _ = tx => println!("Sender task completed!"),
        _ = rx => println!("Receiver task completed!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comm_channel() {
        comm_channel();
    }

    #[tokio::test]
    async fn test_tokio_channels() {
        tokio_channels().await;
    }
}

```