use std::{
    hint,
    mem::MaybeUninit,
    sync::atomic::{
        AtomicUsize,
        Ordering::{Acquire, Relaxed, Release},
    },
};

pub struct SpscChannel<T> {
    buffer: Box<[MaybeUninit<T>]>,
    capacity: usize,
    head: AtomicUsize,
    tail: AtomicUsize,
}

unsafe impl<T> Sync for SpscChannel<T> where T: Send {}

impl<T> SpscChannel<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than zero");

        let mut buffer = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            buffer.push(MaybeUninit::uninit());
        }
        Self {
            buffer: buffer.into_boxed_slice(),
            capacity,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn send(&mut self, item: T) -> Result<(), T> {
        if self.capacity == self.buffer.len() {
            hint::spin_loop();
            return Err(item);
        } else {
            let tail = self.tail.load(Acquire);
            let new_tail = tail + 1;
            if let Some(unit) = self.buffer.get_mut(new_tail) {
                unit.write(item);
            }

            while self
                .tail
                .compare_exchange(tail, new_tail, Release, Relaxed)
                .is_err()
            {
                hint::spin_loop();
            }

            return Ok(());
        }
    }

    pub fn recv(&mut self) -> Option<T> {}
}
