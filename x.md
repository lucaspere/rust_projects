### Threads
In rust we can spawn threads using ``std::thread::spawn``. The ``main`` function execute the main thread, if we spawn another thread, it'll execute in another context and the main thread can continue. If we want to the main thread to wait the child thread, we need to ``join`` the threads.
### Shared and Exclusive References (& and &mut)
Rust allows multiples references but only one mutable reference, when use a reference, Rust does not copy the value because it's actually a reference of its memory address.
### Mutexes
To avoid data race, rust only allows one variable to hold and modify a mutable value. Since threads can run in parallel, we need to use Mutex. To get a mutable value to modify, we use ``lock`` and the compiler will lock other threads that are waiting for use this variabble. This guarantee thread safety and avoid data race. Also, to get multiples sharable values, we can use RwLock, that enables multiples threads to use a sharable value at the same time.
### Send and Sync
These traits are using to tell the compiler that the data struct that is not natively thread safe, like structured data, is safe to send or share through threads. The send means the data can be send to another thread like ``Arc<i32>`` aand Sync tells that the data can be share like ``i32``. A struct with fields that are all Send and Sync, is itself also Send and Sync.
### Panic and Mutex
A Mutex in Rust gets marked as poisoned when a thread panics while holding the lock. When this happened, a thread that is trying to acquire a mutex calling ``lock`` receive a ``Err`` instead of the value. This enforces that the user to handle this situation, since the ``lock`` returns a MutexGuard to safely acquire a value. 
### Cell vs. RefCell Choice
Cell only allows us to copy the value or replace it with another value as a whole. In different way, the RefCell holds a counter, so we can borrow its contents, at a small runtime cost. The Cell value we cannot borrow, so we cannot take an exclusive reference.