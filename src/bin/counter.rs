use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let counter = std::sync::atomic::AtomicU64::new(0);

    std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10000000 {
                counter.fetch_add(1, Relaxed);
            }
        });
        for _ in 0..10000000 {
            counter.fetch_add(1, Relaxed);
        }
    });
    assert_eq!(20000000, counter.load(Relaxed));
    println!("Counter = {}", counter.load(Relaxed));
}