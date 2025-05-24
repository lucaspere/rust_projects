use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicBool};

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    in_use: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
            in_use: AtomicBool::new(false),
        }
    }

    /// Panics when trying to send more than one message.
    pub fn send(&self, message: T) {
        if !self.in_use.swap(true, Relaxed) {
            panic!("can't send more than one message!");
        }

        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    /// Panic if no message is available yet,
    /// or if the message was already consumed.
    pub fn receive(&self) -> T {
        if !self.ready.swap(false, Acquire) {
            panic!("no message available!");
        }

        // Safety: we've just check (and reset) the ready flag.
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
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
        let channel = Channel::new();

        let t = thread::current();

        thread::scope(|s| {
            s.spawn(|| {
                channel.send("Hello world!");
                t.unpark();
            });

            while !channel.is_ready() {
                thread::park();
            }

            assert_eq!(channel.receive(), "helo world!");
        });
    }
}
