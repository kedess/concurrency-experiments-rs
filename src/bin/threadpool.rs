use std::time::Instant;

use concurrency_experiments_rs::threadpool::ThreadPool;

static mut COUNTER: u32 = 0;

fn main() {
    let start = Instant::now();
    let mut pool = ThreadPool::new(6);
    pool.run();
    for _ in 0..10000 {
        pool.submit(Box::new(|| unsafe {
            let it = random(113);
            for value in it.take(1000000) {
                COUNTER += value % 100;
            }
        }));
    }
    pool.wait();
    println!(
        "COUNTER = {}, elapsed time {} ms",
        unsafe { COUNTER },
        start.elapsed().as_millis()
    );
    pool.stop();
}

fn random(seed: u32) -> impl Iterator<Item = u32> {
    let mut value = seed;
    std::iter::repeat_with(move || {
        value ^= value << 13;
        value ^= value >> 17;
        value ^= value << 5;
        value
    })
}
