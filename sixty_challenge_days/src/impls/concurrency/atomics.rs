use std::{
    io::stdin,
    ptr::null_mut,
    sync::{
        atomic::{
            AtomicBool, AtomicI32, AtomicPtr, AtomicU64, AtomicUsize,
            Ordering::{Acquire, Relaxed, Release},
        },
        Once,
    },
    thread::{self, sleep},
    time::{Duration, Instant},
};

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.store(10, Relaxed);
    Y.store(20, Relaxed);
}

fn b() {
    let y = Y.load(Relaxed);
    let x = X.load(Relaxed);
    println!("{x} {y}");
}

pub fn atomic_ordering() {
    let t1 = thread::spawn(|| a());
    let t2 = thread::spawn(|| b());

    t1.join().unwrap();
    t2.join().unwrap();
}

pub fn stop_flag() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let background_tread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            println!("Background thread is running...");
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    thread::spawn(|| {
        for line in stdin().lines() {
            match line.unwrap().as_str() {
                "help" => println!("commands: help, stop"),
                "stop" => break,
                cmd => println!("Unknown command: {:?}", cmd),
            }
        }
    });

    STOP.store(true, std::sync::atomic::Ordering::Relaxed);

    background_tread.join().unwrap();
}

pub fn report() {
    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();
    thread::scope(|s| {
        // A background thread to process all 100 items.
        s.spawn(|| {
            for i in 0..100 {
                process_item(i); // Assuming this takes some time.
                num_done.store(i + 1, Relaxed);
                main_thread.unpark(); // Wake up the main thread.
            }
        });

        // The main thread shows status updates.
    });
    loop {
        thread::park_timeout(Duration::from_secs(1));
        let n = num_done.load(Relaxed);
        println!("Working.. {}/100 done", n);
        if n == 100 {
            break;
        }
        println!("Working.. {n}/100 done");
    }

    println!("Done!");
}

pub fn mult_report() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i); // Assuming this takes some time.
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        // The main thread shows status updates, every second.
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            if n == 0 {
                println!("Working.. nothing done yet.");
            } else {
                println!(
                    "Working.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time,
                );
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}
fn process_item(_i: usize) {}

pub fn lazy_eval() {
    static VAL: AtomicU64 = AtomicU64::new(0);

    let mut val = VAL.load(Relaxed);
    if val == 0 {
        val = 42;
        VAL.store(val, Relaxed);
    }
}

const START: Once = Once::new();
pub fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Relaxed);
    if key == 0 {
        START.call_once(|| {
            let new_key = generate_key();

            match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
                Ok(_) => new_key,
                Err(k) => k,
            };
        });
        KEY.load(Relaxed)
    } else {
        key
    }
}

fn generate_key() -> u64 {
    42
}

fn atomic_release_acquire() {
    static DATA: AtomicU64 = AtomicU64::new(0);
    static READY: AtomicBool = AtomicBool::new(false);

    thread::spawn(|| {
        DATA.fetch_add(123, Relaxed);

        READY.store(true, Release);
    });

    while !READY.load(Relaxed) {
        sleep(Duration::from_millis(500));

        println!("Waiting...");
    }

    println!("{}", DATA.load(Relaxed));
}

struct Data {
    name: String,
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));

        if let Err(e) = PTR.compare_exchange(null_mut(), p, Release, Acquire) {
            drop(unsafe { Box::from_raw(p) });

            p = e;
        }
    }

    unsafe { &*p }
}

fn generate_data() -> Data {
    Data {
        name: "Lucas".to_string(),
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_stop_flag() {
        stop_flag();
    }

    #[test]
    fn test_report() {
        report();
    }

    #[test]
    fn test_mult_report() {
        mult_report();
    }

    #[test]
    fn test_lazy_eval() {
        lazy_eval();
    }

    #[test]
    fn test_get_key() {
        let key = get_key();
        let key2 = get_key();

        assert_eq!(key, key2);
    }

    #[test]
    fn test_atomic_ordering() {
        atomic_ordering();
    }

    #[test]
    fn test_atomic_() {
        atomic_release_acquire();
    }

    #[test]
    fn test_atomic_ptr() {
        get_data();
    }
}
