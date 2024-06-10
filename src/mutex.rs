use std::sync::atomic::{AtomicU64, Ordering};

pub struct SpinLock {
    owner_ticket: AtomicU64,
    ticket: AtomicU64,
}

unsafe impl Send for SpinLock {}
unsafe impl Sync for SpinLock {}

impl SpinLock {
    pub fn new() -> Self {
        SpinLock {
            owner_ticket: AtomicU64::new(0),
            ticket: AtomicU64::new(0),
        }
    }
    pub fn lock(&self) {
        let ticket = self.ticket.fetch_add(1, Ordering::SeqCst);
        while self.owner_ticket.load(Ordering::SeqCst) != ticket {}
    }
    pub fn unlock(&self) {
        self.owner_ticket.fetch_add(1, Ordering::SeqCst);
    }
}
