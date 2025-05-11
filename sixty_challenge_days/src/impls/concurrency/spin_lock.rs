use std::hint;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

trait Lock {
    fn lock(&self);
    fn unlock(&self);
}
struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub fn new() -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
        }
    }
}

impl Lock for SpinLock {
    fn lock(&self) {
        while self
            .locked
            .compare_exchange(false, true, Acquire, Relaxed)
            .is_err()
        {
            hint::spin_loop();
        }
    }

    fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

#[cfg(test)]
mod tests {

    use std::sync::atomic::Ordering::Relaxed;
    use std::{
        sync::{atomic::AtomicUsize, Arc},
        thread,
    };

    use super::{Lock, SpinLock};

    #[test]
    fn test_spin_lock() {
        let spin_lock = Arc::new(SpinLock::new());
        let protected_counter = Arc::new(AtomicUsize::new(0));

        let num_threads = 10;
        let increments_per_thread = 10_000;
        let mut handles = vec![];

        for i in 0..num_threads {
            let lock_clone = Arc::clone(&spin_lock);
            let counter_clone = Arc::clone(&protected_counter);

            let handle = thread::spawn(move || {
                for j in 0..increments_per_thread {
                    lock_clone.lock();

                    let current = counter_clone.load(Relaxed);
                    counter_clone.store(current + 1, Relaxed);

                    if j % 2_000 == 0 && j > 0 && i == 0 {
                        println!("Thread {} na iteração {}", i, j);
                    }

                    lock_clone.unlock();
                }
            });

            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        let final_value = protected_counter.load(Relaxed);
        println!(
            "Contador protegido final (com Acquire/Release): {}",
            final_value
        );
        assert_eq!(final_value, num_threads * increments_per_thread);
        println!("Verificação do contador protegido (Acquire/Release) passou!");
    }
}
