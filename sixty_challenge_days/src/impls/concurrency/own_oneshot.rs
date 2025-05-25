use std::marker::PhantomData;
use std::sync::atomic::AtomicBool;
use std::thread::{self, Thread};
use std::{cell::UnsafeCell, mem::MaybeUninit};

use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub struct Sender<'a, T> {
    oneshot: &'a Oneshot<T>,
    receiving_thread: Thread,
}
pub struct Receiver<'a, T> {
    oneshot: &'a Oneshot<T>,
    _no_send: PhantomData<*const ()>,
}

impl<'a, T> Sender<'a, T> {
    pub fn send(self, message: T) {
        unsafe { (*self.oneshot.message.get()).write(message) };
        self.oneshot.is_ready.store(true, Release);
        self.receiving_thread.unpark();
    }
}

impl<'a, T> Receiver<'a, T> {
    pub fn is_ready(&self) -> bool {
        self.oneshot.is_ready.load(Relaxed)
    }

    pub fn receive(&self) -> T {
        while !self.oneshot.is_ready.swap(false, Acquire) {
            thread::park();
        }

        // Safety: we've just check (and reset) the ready flag.
        unsafe { (*self.oneshot.message.get()).assume_init_read() }
    }
}

pub struct Oneshot<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    is_ready: AtomicBool,
}

unsafe impl<T> Sync for Oneshot<T> where T: Send {}

impl<T> Oneshot<T> {
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
                oneshot: self,
                receiving_thread: thread::current(),
            },
            Receiver {
                oneshot: self,
                _no_send: PhantomData,
            },
        )
    }
}
impl<T> Drop for Oneshot<T> {
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
    use std::time::Duration;

    use super::Oneshot;

    #[test]
    fn test_channel_send_receive_with_thread_parking() {
        let mut oneshot = Oneshot::new();

        thread::scope(|s| {
            let (sender, receiver) = oneshot.split();
            s.spawn(|| {
                sender.send("Hello world!");
            });

            assert_eq!(receiver.receive(), "Hello world!");
        });
    }

    #[test]
    fn test_multiple_messages() {
        let mut oneshot1 = Oneshot::new();
        let mut oneshot2 = Oneshot::new();

        thread::scope(|s| {
            let (sender1, receiver1) = oneshot1.split();
            let (sender2, receiver2) = oneshot2.split();

            s.spawn(move || {
                sender1.send(42);
            });

            s.spawn(move || {
                sender2.send(84);
            });

            assert_eq!(receiver1.receive(), 42);
            assert_eq!(receiver2.receive(), 84);
        });
    }

    #[test]
    fn test_channel_with_complex_type() {
        let mut oneshot = Oneshot::new();

        thread::scope(|s| {
            let (sender, receiver) = oneshot.split();
            s.spawn(|| {
                sender.send(vec![1, 2, 3, 4]);
            });

            assert_eq!(receiver.receive(), vec![1, 2, 3, 4]);
        });
    }

    #[test]
    fn test_is_ready_method() {
        let mut oneshot = Oneshot::new();

        thread::scope(|s| {
            let (sender, receiver) = oneshot.split();

            // Initially not ready
            assert!(!receiver.is_ready());

            s.spawn(move || {
                thread::sleep(Duration::from_millis(10));
                sender.send("Data");
            });

            // Block until ready
            while !receiver.is_ready() {
                thread::park_timeout(Duration::from_millis(1));
            }

            assert!(receiver.is_ready());
            assert_eq!(receiver.receive(), "Data");
            // After receiving, should not be ready again
            assert!(!receiver.is_ready());
        });
    }
}
