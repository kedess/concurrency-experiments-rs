use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Mutex;

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
    // std::thread::scope(|s| {
    //     s.spawn(|| {
    //         for _ in 0..10 {
    //             println!("Thread 1, id = {}", generator_id());
    //             std::thread::sleep(Duration::from_millis(20));
    //         }
    //     });
    //     s.spawn(|| {
    //         for _ in 0..10 {
    //             println!("Thread 2, id = {}", generator_id());
    //             std::thread::sleep(Duration::from_millis(20));
    //         }
    //     });
    //     s.spawn(|| {
    //         for _ in 0..10 {
    //             println!("Thread 3, id = {}", generator_id_check());
    //             std::thread::sleep(Duration::from_millis(20));
    //         }
    //     });
    //     s.spawn(|| {
    //         for _ in 0..10 {
    //             println!("Thread 4, id = {}", generator_id_check());
    //             std::thread::sleep(Duration::from_millis(20));
    //         }
    //     });
    // });

    let start = std::time::Instant::now();
    std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10000000 {
                generator_id();
            }
        });
        s.spawn(|| {
            for _ in 0..10000000 {
                generator_id();
            }
        });
    });
    assert_eq!(20000001, generator_id());
    println!(
        "Generator atomic = elapsed {} ms",
        start.elapsed().as_millis()
    );

    let start = std::time::Instant::now();
    std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10000000 {
                generator_id_check();
            }
        });
        s.spawn(|| {
            for _ in 0..10000000 {
                generator_id_check();
            }
        });
    });
    assert_eq!(20000001, generator_id_check());
    println!(
        "Generator atomic with check = elapsed {} ms",
        start.elapsed().as_millis()
    );

    let mutex = Mutex::new(0);
    let start = std::time::Instant::now();
    std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10000000 {
                let mut guard = mutex.lock().unwrap();
                *guard += 1;
            }
        });
        s.spawn(|| {
            for _ in 0..10000000 {
                let mut guard = mutex.lock().unwrap();
                *guard += 1;
            }
        });
    });
    assert_eq!(20000001, *mutex.lock().unwrap() + 1);
    println!(
        "Generator on mutex = elapsed {} ms",
        start.elapsed().as_millis()
    );
}
