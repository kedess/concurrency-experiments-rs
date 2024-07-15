use std::{collections::VecDeque, thread::JoinHandle};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc, Mutex,
    },
    time::Duration,
};

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    size: usize,
    workers: Vec<JoinHandle<()>>,
    queue: Arc<Mutex<VecDeque<Task>>>,
    is_stop: Arc<AtomicBool>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        ThreadPool {
            size,
            workers: vec![],
            queue: Default::default(),
            is_stop: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn run(&mut self) {
        for _ in 0..self.size {
            let queue = Arc::clone(&self.queue);
            let is_stop = Arc::clone(&self.is_stop);
            self.workers.push(std::thread::spawn(move || {
                while !is_stop.load(Relaxed) {
                    let mut guard = queue.lock().unwrap();
                    if let Some(task) = guard.pop_front() {
                        drop(guard);
                        task();
                    } else {
                        drop(guard);
                        std::thread::sleep(Duration::from_millis(10));
                    }
                }
            }));
        }
    }
    pub fn submit(&mut self, func: Task) {
        let mut guard = self.queue.lock().unwrap();
        guard.push_back(func);
    }
    pub fn stop(&mut self) {
        self.is_stop.store(true, Relaxed);
        while !self.workers.is_empty() {
            let worker = self.workers.pop().unwrap();
            worker.join().unwrap();
        }
        println!("ThreadPool is stopped");
    }
    pub fn wait(&self) {
        while !self.queue.lock().unwrap().is_empty() {}
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if !self.is_stop.load(Relaxed) {
            self.stop();
        }
    }
}
