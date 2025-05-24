use std::sync::atomic::AtomicBool;
use std::{cell::UnsafeCell, mem::MaybeUninit};

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
}
pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
}

impl<'a, T> Sender<'a, T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.is_ready.store(true, Release);
    }
}

impl<'a, T> Receiver<'a, T> {
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

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    is_ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            is_ready: AtomicBool::new(false),
        }
    }

    pub fn split(&mut self) -> (Sender<T>, Receiver<T>) {
        *self = Self::new();

        (Sender { channel: self }, Receiver { channel: self })
    }
}
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

    use super::Channel;

    #[test]
    fn test_channel_send_receive_with_thread_parking() {
        let mut channel = Channel::new();
        let t = thread::current();

        thread::scope(|s| {
            let (sender, receiver) = channel.split();
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
