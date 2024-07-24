use std::sync::atomic::Ordering::Relaxed;
use std::{sync::atomic::AtomicU64, time::Duration};

fn generator_id() -> u64 {
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);
    NEXT_ID.fetch_add(1, Relaxed)
}
fn generator_id_check() -> u64 {
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);
    let mut id = NEXT_ID.load(Relaxed);
    loop {
        assert!(id < u64::MAX);
        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => return id,
            Err(v) => id = v,
        }
    }
}

fn main() {
    std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10 {
                println!("Thread 1, id = {}", generator_id());
                std::thread::sleep(Duration::from_millis(20));
            }
        });
        s.spawn(|| {
            for _ in 0..10 {
                println!("Thread 2, id = {}", generator_id());
                std::thread::sleep(Duration::from_millis(20));
            }
        });
        s.spawn(|| {
            for _ in 0..10 {
                println!("Thread 3, id = {}", generator_id_check());
                std::thread::sleep(Duration::from_millis(20));
            }
        });
        s.spawn(|| {
            for _ in 0..10 {
                println!("Thread 4, id = {}", generator_id_check());
                std::thread::sleep(Duration::from_millis(20));
            }
        });
    });
}
