use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::{
    collections::HashMap,
    sync::atomic::AtomicU64,
    sync::{Once, OnceLock},
};

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
}
