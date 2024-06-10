use std::sync::Arc;

mod mutex;

fn main() {
    let spin_lock = Arc::new(mutex::SpinLock::new(0));
    let spin_lock_first = spin_lock.clone();
    let spin_lock_second = spin_lock.clone();

    let th1 = std::thread::spawn(move || {
        for _ in 0..10000000 {
            let mut guard = spin_lock_first.guard();
            *guard += 1;
        }
    });
    let th2 = std::thread::spawn(move || {
        for _ in 0..10000000 {
            let mut guard = spin_lock_second.guard();
            *guard += 1;
        }
    });
    let _ = th1.join();
    let _ = th2.join();
    println!("sum = {}", *spin_lock.guard());
}
