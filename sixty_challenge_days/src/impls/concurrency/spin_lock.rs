use std::cell::UnsafeCell;
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

pub struct LockedData<T> {
    lock: SpinLock,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for LockedData<T> {}
unsafe impl<T: Sync> Sync for LockedData<T> {}

impl<T> LockedData<T> {
    pub fn new(data: T) -> Self {
        Self {
            lock: SpinLock::new(),
            data: UnsafeCell::new(data),
        }
    }

    pub fn with_lock<F, R>(&self, actor_name: &str, operation: F) -> R
    where
        F: FnOnce(&str, &mut T) -> R,
    {
        self.lock.lock();

        let result = unsafe { operation(actor_name, &mut *self.data.get()) };
        self.lock.unlock();

        result
    }
}
#[cfg(test)]
mod tests {

    use std::{sync::Arc, thread};

    use crate::impls::{
        bank_account::bank_account::BankAccount, concurrency::spin_lock::LockedData,
    };

    #[test]
    fn test_spin_lock() {
        let locked_account = Arc::new(LockedData::new(BankAccount::new(1, 1000)));
        let num_threads = 5;
        let operations_per_thread = 2;
        let mut handles = vec![];

        for i in 0..num_threads {
            let account_clone = locked_account.clone();
            let actor_name = format!("Thread-{}", i);
            let handle = thread::spawn(move || {
                for j in 0..operations_per_thread {
                    if (i + j) % 2 == 0 {
                        let amount_to_deposit = (j * 10 + 50) as i64;
                        account_clone.with_lock(&actor_name, |actor, account| {
                            account.deposit(amount_to_deposit, actor);
                        });
                    } else {
                        let amount_to_withdraw = (j * 10 + 30) as i64;
                        account_clone.with_lock(&actor_name, |actor, account| {
                            account.withdraw(amount_to_withdraw, actor);
                        });
                    }
                }
            });

            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        println!("\n--- Operations Completed ---");
        locked_account.with_lock("main_thread_final_check", |_, account| {
            println!("Final Account: {}", account.id);
            println!("Final Balance: {}", account.balance);
            println!("\nComplete Transaction Log:");
            for entry in &account.transaction_log {
                println!(" - {}", entry);
            }
        });
    }
}
