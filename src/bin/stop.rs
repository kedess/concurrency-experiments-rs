use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);
    let backgound_thread = std::thread::spawn(|| {
        while !STOP.load(Relaxed) {
            println!("Background msg");
            std::thread::sleep(Duration::from_secs(5));
        }
    });
    for msg in std::io::stdin().lines() {
        if msg.unwrap() == "stop" {
            STOP.store(true, Relaxed);
            break;
        }
    }
    backgound_thread.join().unwrap();
}
