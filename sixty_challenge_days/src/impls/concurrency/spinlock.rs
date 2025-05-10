use std::{cell::UnsafeCell, sync::atomic::AtomicBool};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> &mut T {
        while self.locked.swap(true, std::sync::atomic::Ordering::Acquire) {
            std::hint::spin_loop();
        }
        unsafe { &mut *self.value.get() }
    }

    /// Safety: The &mut T from lock() must be gone!
    /// (And no cheating by keeping reference to fields of that T around!)
    pub unsafe fn unlock(&self) {
        self.locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}
