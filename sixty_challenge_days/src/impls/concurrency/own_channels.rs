use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::{cell::UnsafeCell, mem::MaybeUninit};

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        is_ready: AtomicBool::new(false),
    });

    (Sender { channel: a.clone() }, Receiver { channel: a })
}

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}
pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.is_ready.store(true, Release);
    }
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.channel.is_ready.load(Relaxed)
    }

    pub fn receive(&self) -> T {
        if !self.channel.is_ready.swap(false, Acquire) {
            panic!("no message available!");
        }

        // Safety: we've just check (and reset) the ready flag.
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    is_ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.is_ready.get_mut() {
            unsafe {
                self.message.get_mut().assume_init_drop();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::thread;

    use super::channel;

    #[test]
    fn test_channel_send_receive_with_thread_parking() {
        let (sender, receiver) = channel();

        let t = thread::current();

        thread::scope(|s| {
            s.spawn(|| {
                sender.send("Hello world!");
                t.unpark();
            });

            while !receiver.is_ready() {
                thread::park();
            }

            assert_eq!(receiver.receive(), "Hello world!");
        });
    }
}
