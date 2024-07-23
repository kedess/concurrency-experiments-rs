fn func() {
    static mut VALUE: i32 = 0;
    unsafe {
        VALUE += 1;
        println!("VALUE = {}", VALUE);
    }
}

fn main() {
    for _ in 0..10 {
        std::thread::spawn(func).join().unwrap();
    }
}
