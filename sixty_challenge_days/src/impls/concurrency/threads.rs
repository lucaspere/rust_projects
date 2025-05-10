use std::{
    ops::Deref,
    sync::{Arc, Mutex, RwLock},
    thread::{self, sleep},
    time::Duration,
};

#[derive(Debug)]
struct SendedThread {
    number: *mut i32,
}

impl Drop for SendedThread {
    fn drop(&mut self) {
        unsafe {
            *self.number = 0;
        }
    }
}

unsafe impl Send for SendedThread {}
unsafe impl Sync for SendedThread {}
impl Deref for SendedThread {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.number }
    }
}

pub fn simple_thread() {
    let mut threads = vec![];
    for number in 0..100 {
        let thread = thread::spawn(move || {
            println!("Hello from a thread! The number is: {:#?}", number);
            sleep(Duration::from_secs(1));
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}

pub fn scoped_threads() {
    let sended = SendedThread { number: &mut 10 };
    thread::scope(|s| {
        s.spawn(|| {
            println!("Hello from a scoped thread! The number is: {:#?}", *sended);
        });

        s.spawn(|| {
            println!(
                "Hello from another scoped thread! The number is: {:#?}",
                *sended
            );
        });
    });
}

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
    let b_thread: thread::JoinHandle<()> = thread::spawn({
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

pub fn mul_read_one_write() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let data = Arc::new(RwLock::new(numbers));

    let a_thread = thread::spawn({
        let data = data.clone();
        move || {
            let mut data = data.write().unwrap();
            data.push(11);
            println!("Hello from a thread! The number is: {:#?}", data);
        }
    });

    let b_thread = thread::spawn({
        let data = data.clone();
        move || {
            let data = data.read().unwrap();
            println!("Hello from another thread! The number is: {:#?}", data);
        }
    });

    let c_thread = thread::spawn({
        let data = data.clone();
        move || {
            let mut data = data.write().unwrap();
            data.push(12);
            println!("Hello from another thread! The number is: {:#?}", data);
        }
    });

    a_thread.join().unwrap();
    b_thread.join().unwrap();
    c_thread.join().unwrap();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_thread() {
        simple_thread();
    }

    #[test]
    fn test_scoped_threads() {
        scoped_threads();
    }

    #[test]
    fn test_shared_owned() {
        shared_owned();
    }

    #[test]
    fn test_mul_read_one_write() {
        mul_read_one_write();
    }
}
