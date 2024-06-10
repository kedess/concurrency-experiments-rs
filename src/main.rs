use std::sync::Arc;

mod mutex;

static mut SUM: u64 = 0;

fn main() {
    let spin_lock = Arc::new(mutex::SpinLock::new());
    let spin_lock_other = spin_lock.clone();

    let th1 = std::thread::spawn(move || {
        for _ in 0..10000000 {
            spin_lock.lock();
            unsafe {
                SUM += 1;
            }
            spin_lock.unlock();
        }
    });
    let th2 = std::thread::spawn(move || {
        for _ in 0..10000000 {
            spin_lock_other.lock();
            unsafe {
                SUM += 1;
            }
            spin_lock_other.unlock();
        }
    });
    let _ = th1.join();
    let _ = th2.join();
    unsafe {
        println!("sum = {}", SUM);
    }
}
