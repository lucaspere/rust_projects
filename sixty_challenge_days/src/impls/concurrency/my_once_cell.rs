use std::hint;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::{cell::UnsafeCell, mem::MaybeUninit, sync::atomic::AtomicUsize};

const EMPTY: usize = 0;
const INITIALIZING: usize = 1;
const INITIALIZED: usize = 2;

pub struct MyOnceCell<T> {
    data: UnsafeCell<MaybeUninit<T>>,
    state: AtomicUsize,
}

unsafe impl<T: Send> Send for MyOnceCell<T> {}
unsafe impl<T: Send + Sync> Sync for MyOnceCell<T> {}

impl<T> MyOnceCell<T> {
    pub fn new() -> Self {
        Self {
            data: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicUsize::new(EMPTY),
        }
    }

    pub fn get_or_init(&self, f: impl FnOnce() -> T) -> &T {
        if self.state.load(Acquire) == INITIALIZED {
            unsafe { (*self.data.get()).assume_init_ref() }
        } else {
            loop {
                let current_loaded_state = self.state.load(Acquire);
                if current_loaded_state == INITIALIZED {
                    return unsafe { (*self.data.get()).assume_init_ref() };
                }

                match self
                    .state
                    .compare_exchange_weak(EMPTY, INITIALIZING, Acquire, Acquire)
                {
                    Ok(_) => {
                        let value = f();

                        unsafe { (*self.data.get()).write(value) };

                        self.state.store(INITIALIZED, Release);
                        return unsafe { (*self.data.get()).assume_init_ref() };
                    }
                    Err(state_val_during_cas_failure) => {
                        if state_val_during_cas_failure == INITIALIZING
                            || state_val_during_cas_failure == EMPTY
                        {
                            hint::spin_loop();
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::MyOnceCell;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_new_is_empty() {
        let cell: MyOnceCell<u32> = MyOnceCell::new();
        // We don't have a way to check directly if it's empty,
        // but we can initialize it and verify it works
        assert_eq!(*cell.get_or_init(|| 42), 42);
    }

    #[test]
    fn test_get_or_init_returns_same_value() {
        let cell = MyOnceCell::new();
        let value1 = cell.get_or_init(|| 42);
        let value2 = cell.get_or_init(|| 84); // This should be ignored

        assert_eq!(*value1, 42);
        assert_eq!(*value2, 42); // Should still be the first value
    }

    #[test]
    fn test_init_runs_once() {
        let cell = MyOnceCell::new();
        let mut counter = 0;

        let value = cell.get_or_init(|| {
            counter += 1;
            counter
        });

        assert_eq!(*value, 1);

        // This should not run the closure again
        let value = cell.get_or_init(|| {
            counter += 1;
            counter
        });

        assert_eq!(*value, 1);
        assert_eq!(counter, 1); // Counter should still be 1
    }

    #[test]
    fn test_multithreaded_init() {
        let cell = Arc::new(MyOnceCell::new());
        let threads = 10;
        let barrier = Arc::new(Barrier::new(threads));
        let mut handles = Vec::new();

        for i in 0..threads {
            let cell_clone = Arc::clone(&cell);
            let barrier_clone = Arc::clone(&barrier);

            handles.push(thread::spawn(move || {
                barrier_clone.wait(); // Wait for all threads to be ready
                let value = cell_clone.get_or_init(|| i);
                *value // Return the value seen by this thread
            }));
        }

        let results: Vec<usize> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All threads should see the same value
        let first = results[0];
        for result in results {
            assert_eq!(result, first);
        }
    }

    #[test]
    fn test_with_complex_type() {
        struct ComplexType {
            value: String,
        }

        let cell = MyOnceCell::new();
        let result = cell.get_or_init(|| ComplexType {
            value: "hello world".to_string(),
        });

        assert_eq!(result.value, "hello world");
    }
}
