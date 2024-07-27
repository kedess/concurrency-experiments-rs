use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::{
    collections::HashMap,
    sync::atomic::AtomicU64,
    sync::{Once, OnceLock},
};

use std::sync::LazyLock;

static DICT: LazyLock<HashMap<i32, i32>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(1, 1);
    map.insert(2, 2);
    map
});

static mut VALUE: usize = 0;
static INIT: Once = Once::new();

fn lazy_init_value() -> usize {
    unsafe {
        INIT.call_once(|| {
            VALUE = 1;
        });
        VALUE
    }
}

fn lazy_init_value_atomic() -> u64 {
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

fn lazy_init_ref() -> &'static HashMap<i32, i32> {
    static MAP: OnceLock<HashMap<i32, i32>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(1, 1);
        map.insert(2, 2);
        map
    })
}

fn lazy_init_ref_atomic() -> &'static HashMap<i32, i32> {
    static PTR: AtomicPtr<HashMap<i32, i32>> = AtomicPtr::new(std::ptr::null_mut());
    let mut ptr = PTR.load(Acquire);
    if ptr.is_null() {
        let mut dict = HashMap::new();
        dict.insert(1, 1);
        dict.insert(2, 2);
        ptr = Box::into_raw(Box::new(dict));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), ptr, Release, Acquire) {
            drop(unsafe { Box::from_raw(ptr) });
            ptr = e;
        }
    }
    unsafe { &*ptr }
}

fn main() {
    let th1 = std::thread::spawn(|| {
        let map = lazy_init_ref();
        let map2 = lazy_init_ref_atomic();
        println!(
            "{} {} {} {}",
            map.get(&1).unwrap(),
            map2.get(&1).unwrap(),
            lazy_init_value(),
            lazy_init_value_atomic()
        );
    });
    let th2 = std::thread::spawn(|| {
        let map = lazy_init_ref();
        let map2 = lazy_init_ref_atomic();
        println!(
            "{} {} {} {}",
            map.get(&1).unwrap(),
            map2.get(&1).unwrap(),
            lazy_init_value(),
            lazy_init_value_atomic()
        );
    });
    th1.join().unwrap();
    th2.join().unwrap();

    let mut value_time = u128::MAX;
    let mut value_atomic_time = u128::MAX;
    let mut reference_time = u128::MAX;
    let mut reference_atomic_time = u128::MAX;
    let mut lazy_static_time = u128::MAX;

    for _ in 0..10 {
        let start = std::time::Instant::now();
        for _ in 0..100000000 {
            let map = lazy_init_ref();
            assert_eq!(1, *map.get(&1).unwrap());
        }
        reference_time = std::cmp::min(reference_time, start.elapsed().as_millis());

        let start = std::time::Instant::now();
        for _ in 0..100000000 {
            let map = lazy_init_ref_atomic();
            assert_eq!(1, *map.get(&1).unwrap());
        }
        reference_atomic_time = std::cmp::min(reference_atomic_time, start.elapsed().as_millis());

        let start = std::time::Instant::now();
        for _ in 0..100000000 {
            let value = lazy_init_value();
            assert_eq!(1, value);
        }
        value_time = std::cmp::min(value_time, start.elapsed().as_millis());

        let start = std::time::Instant::now();
        for _ in 0..100000000 {
            let value = lazy_init_value_atomic();
            assert_eq!(1, value);
        }
        value_atomic_time = std::cmp::min(value_atomic_time, start.elapsed().as_millis());

        let start = std::time::Instant::now();
        for _ in 0..100000000 {
            let value = DICT.get(&1).unwrap();
            assert_eq!(1, *value);
        }
        lazy_static_time = std::cmp::min(lazy_static_time, start.elapsed().as_millis());
    }
    println!("OnceLock = elapsed {} ms", reference_time);
    println!("Atomic reference = elapsed {} ms", reference_atomic_time);
    println!("Once = elapsed {} ms", value_time);
    println!("Atomic value = elapsed {} ms", value_atomic_time);
    println!("Lazy lock = elapsed {} ms", lazy_static_time);
}
