use std::marker::PhantomData;
use std::sync::atomic::AtomicBool;
use std::thread::{self, Thread};
use std::{cell::UnsafeCell, mem::MaybeUninit};

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receiving_thread: Thread,
}
pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    _no_send: PhantomData<*const ()>,
}

impl<'a, T> Sender<'a, T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.is_ready.store(true, Release);
        self.receiving_thread.unpark();
    }
}

impl<'a, T> Receiver<'a, T> {
    pub fn is_ready(&self) -> bool {
        self.channel.is_ready.load(Relaxed)
    }

    pub fn receive(&self) -> T {
        while !self.channel.is_ready.swap(false, Acquire) {
            thread::park();
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

        (
            Sender {
                channel: self,
                receiving_thread: thread::current(),
            },
            Receiver {
                channel: self,
                _no_send: PhantomData,
            },
        )
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

        thread::scope(|s| {
            let (sender, receiver) = channel.split();
            s.spawn(|| {
                sender.send("Hello world!");
            });

            assert_eq!(receiver.receive(), "Hello world!");
        });
    }
}
