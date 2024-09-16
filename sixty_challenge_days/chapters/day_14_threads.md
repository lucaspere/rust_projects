# Day 14: Threads and Mutexes

## Threads
A process is an instancia to be executed by operating system. Differents process doens't share memory contextos like memory and to communicate each other, they have to ask the kernel first. However, a process can spawn threads that share the same process context and can communicate each other without asking the kernel.

### Threads in Rust
You can use threads in Rust using the core library ``std::threads``. To spawn one thread, use the method ``spawn``.

#### Basic Thread Spawning: Write a simple Rust program that creates multiple threads, each printing a message. Observe how they execute concurrently.
```rs
use std::thread;

fn main() {
    let sub_thr = thread::spawn(|| {
        let id = thread::current().id()
        println!("Hello from the sub thread, my ID is: {id}");
    })

    let id = thread::current().id()
    println!("Hello from the main thread, my ID is: {id}");
}
```

#### Shared Mutable State and Mutex Protection
```rs
pub fn shared_owned() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let shared_owner = Arc::new(Mutex::new(numbers));

    let a_thread = thread::spawn({
        let shared_owner = shared_owner.clone();
        move || {
            let mut shared_owner = shared_owner.lock().unwrap();
            shared_owner.push(11);
            println!("Hello from a thread! The number is: {:#?}", shared_owner);
        }
    });
    let b_thread = thread::spawn({
        let shared_owner = shared_owner.clone();
        move || {
            let mut shared_owner = shared_owner.lock().unwrap();
            shared_owner.push(12);
            println!(
                "Hello from another thread! The number is: {:#?}",
                shared_owner
            );
        }
    });
    a_thread.join().unwrap();
    b_thread.join().unwrap();
}
```

#### Discussion & Questions
##### Thread Safety: What is the difference between thread-safe and non-thread-safe code? Why is it crucial in concurrent programming?
The different is that thread-safe is when the compiler knows the code behavior and can prevent some concurrent issues like deadlock and data racing. For example, send between threads the struct ``struct Product {number: i32}`` is thread-safe because the ``i32`` implement ``Send`` and ``Sync`` traits, so the enteire struct is thread-safe. However, code like 
```rs
struct Product {
    number_ptr: *mut i32
}

unsafe impl Send for Product
unsafe impl Sync for Product
```
is non-thread-safe because the compiler does not know what the raw pointer is referent to, so this is unsafe code and could happen **undefined behavior**

##### Mutex Behavior: How does a ``Mutex`` prevent data races? What happens if multiple threads try to acquire the lock at the same time?
With ``Mutex``, we can adquire an **exclusive reference (&mut T)**, so we can change the value locked. Mutex prevent data races by tagging the critical value as _locked_ and _unlocked_: when a value is locked by one thread, others threads are put to sleep until the other thread unlock the value. Threads only can attempt unlocked values, and it only can be unlocked by the threads that adquired the lock.

##### Deadlocks: What are potential scenarios where deadlocks could occur in your code? How can you design your program to avoid them?
When the process locks two Mutex and two threads is waiting the release one of them to wake up. In Rust, a lock value is release in end of the scoped, the compiler drop it automatically. So to prevent the deadlock is important to drop it as soon as possible to avoid deadlock and delay using the function ``drop``.


#### RwLock, condition variables (Condvar) and Thread Parking
##### RwLock
Mutex only lock exclusive reference, so we can change the data whenever we lock the value. RwLock, adquire both **shared reference (&T, readers) and exclusive (writers). So we can have many threads reading the value without sleeping, but when one thread is writting, all others are put to sleep until it releses. Condvar (Condition variables) enable threads to wait for a specific condition to become true before proceeding, useful for synchronization and coordination. ``thread::park`` puts a thread to sleep until it's explicitly woken up, providing a way for threads to wait for external events or signals.


## References
1. Bos, Mara; Rust Atomics and Locks 1ed. Chapter 1: Basics of Rust Concurrency.
    