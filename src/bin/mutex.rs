use std::sync::Mutex;

use concurrency_experiments_rs::mutex::{SpinLock, SpinLockTicket};

fn main() {
    let mut spinlock_ticket_time = u128::MAX;
    let mut spinlock_time = u128::MAX;
    let mut mutex_time = u128::MAX;
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
    }

    println!("SpinlockTicket = elapsed {} ms", spinlock_ticket_time);
    println!("Spinlock = elapsed {} ms", spinlock_time);
    println!("Mutex = elapsed {} ms", mutex_time);
}
