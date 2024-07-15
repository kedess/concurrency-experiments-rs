use std::sync::{
    Arc, Condvar, Mutex,
};
use std::{collections::VecDeque, thread::JoinHandle};

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    size: usize,
    workers: Vec<JoinHandle<()>>,
    queue: Arc<Mutex<VecDeque<Task>>>,
    not_empty: Arc<Condvar>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        ThreadPool {
            size,
            workers: vec![],
            queue: Default::default(),
            not_empty: Arc::new(Condvar::new()),
        }
    }
    pub fn run(&mut self) {
        for _ in 0..self.size {
            let queue = Arc::clone(&self.queue);
            let not_empty = Arc::clone(&self.not_empty);
            self.workers.push(std::thread::spawn(move || loop {
                let mut guard = queue.lock().unwrap();
                let task = loop {
                    if let Some(task) = guard.pop_front() {
                        break task;
                    } else {
                        guard = not_empty.wait(guard).unwrap();
                    }
                };
                drop(guard);
                task();
            }));
        }
    }
    pub fn submit(&mut self, func: Task) {
        let mut guard = self.queue.lock().unwrap();
        guard.push_back(func);
        self.not_empty.notify_one();
    }
    pub fn wait(&self) {
        while !self.queue.lock().unwrap().is_empty() {}
    }
}
