use std::{sync::atomic::AtomicU64, time::Duration};

fn generator_id() -> u64 {
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
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
    });
}
