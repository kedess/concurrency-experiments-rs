static VALUE: i32 = 1;

fn main() {
    let value: &'static i32 = Box::leak(Box::new(77));
    std::thread::spawn(move || {
        println!("{} {}", VALUE, value);
    })
    .join()
    .unwrap();
    std::thread::spawn(move || {
        println!("{} {}", VALUE, value);
    })
    .join()
    .unwrap();
}
