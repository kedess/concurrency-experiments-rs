use std::sync::atomic::Ordering::Relaxed;
use std::{
    collections::HashMap,
    sync::atomic::AtomicU64,
    sync::{Once, OnceLock},
};

static mut VALUE: usize = 0;
static INIT: Once = Once::new();

fn get_value() -> usize {
    unsafe {
        INIT.call_once(|| {
            VALUE = 1;
        });
        VALUE
    }
}

fn get_value_atomic() -> u64 {
    static VALUE: AtomicU64 = AtomicU64::new(0);
    let mut value = VALUE.load(Relaxed);
    if value == 0 {
        value = 1;
        match VALUE.compare_exchange(0, value, Relaxed, Relaxed) {
            Ok(_) => value,
            Err(v) => v,
        }
    } else {
        value
    }
}

fn get_dict() -> &'static HashMap<i32, i32> {
    static MAP: OnceLock<HashMap<i32, i32>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(1, 1);
        map.insert(2, 2);
        map
    })
}

fn main() {
    let th1 = std::thread::spawn(|| {
        let map = get_dict();
        println!(
            "{} {} {}",
            map.get(&1).unwrap(),
            get_value(),
            get_value_atomic()
        );
    });
    let th2 = std::thread::spawn(|| {
        let map = get_dict();
        println!(
            "{} {} {}",
            map.get(&1).unwrap(),
            get_value(),
            get_value_atomic()
        );
    });
    th1.join().unwrap();
    th2.join().unwrap();
}
