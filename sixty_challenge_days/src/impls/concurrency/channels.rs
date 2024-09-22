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
