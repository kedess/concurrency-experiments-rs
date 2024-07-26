use std::sync::{atomic::AtomicBool, Mutex};

use concurrency_rs::mutex::{SpinLock, SpinLockTicket};
static mut DATA: i32 = 0;
static LOCKED: AtomicBool = AtomicBool::new(false);

fn main() {
    let mut spinlock_ticket_time = u128::MAX;
    let mut spinlock_time = u128::MAX;
    let mut mutex_time = u128::MAX;
    let mut mutex_time_atomic = u128::MAX;
    for _ in 0..10 {
        let start = std::time::Instant::now();
        let spin_lock = SpinLockTicket::new(0);
        std::thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..10000000 / 2 {
                    let mut guard = spin_lock.guard();
                    *guard += 1;
                }
            });
            for _ in 0..10000000 / 2 {
                let mut guard = spin_lock.guard();
                *guard += 1;
            }
        });
        assert_eq!(10000000, *spin_lock.guard());
        spinlock_ticket_time = std::cmp::min(spinlock_ticket_time, start.elapsed().as_millis());

        let start = std::time::Instant::now();
        let spin_lock = SpinLock::new(0);
        std::thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..10000000 / 2 {
                    let mut guard = spin_lock.guard();
                    *guard += 1;
                }
            });
            for _ in 0..10000000 / 2 {
                let mut guard = spin_lock.guard();
                *guard += 1;
            }
        });
        assert_eq!(10000000, *spin_lock.guard());
        spinlock_time = std::cmp::min(spinlock_time, start.elapsed().as_millis());

        let start = std::time::Instant::now();
        let mutex = Mutex::new(0);
        std::thread::scope(|s| {
            s.spawn(|| {
                for _ in 0..10000000 / 2 {
                    let mut guard = mutex.lock().unwrap();
                    *guard += 1;
                }
            });
            for _ in 0..10000000 / 2 {
                let mut guard = mutex.lock().unwrap();
                *guard += 1;
            }
        });
        assert_eq!(10000000, *mutex.lock().unwrap());
        mutex_time = std::cmp::min(mutex_time, start.elapsed().as_millis());

        let start = std::time::Instant::now();
        unsafe {
            DATA = 0;
        }
        std::thread::scope(|s| {
            s.spawn(|| {
                let mut cnt = 10000000 / 2;
                while cnt > 0 {
                    if LOCKED.swap(true, std::sync::atomic::Ordering::Acquire) == false {
                        unsafe { DATA += 1 };
                        cnt -= 1;
                        LOCKED.store(false, std::sync::atomic::Ordering::Release);
                    }
                }
            });
            let mut cnt = 10000000 / 2;
            while cnt > 0 {
                if LOCKED.swap(true, std::sync::atomic::Ordering::Acquire) == false {
                    unsafe { DATA += 1 };
                    cnt -= 1;
                    LOCKED.store(false, std::sync::atomic::Ordering::Release);
                }
            }
        });
        assert_eq!(10000000, unsafe { DATA });
        mutex_time_atomic = std::cmp::min(mutex_time_atomic, start.elapsed().as_millis());
    }

    println!("SpinlockTicket = elapsed {} ms", spinlock_ticket_time);
    println!("Spinlock = elapsed {} ms", spinlock_time);
    println!("Mutex = elapsed {} ms", mutex_time);
    println!("Mutex on atomic = elapsed {} ms", mutex_time);
}
