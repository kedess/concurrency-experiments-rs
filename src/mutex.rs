use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Clone)]
pub struct Guard<T> {
    spin_lock: *const SpinLock<T>,
}

impl<T> Drop for Guard<T> {
    fn drop(&mut self) {
        unsafe { (*self.spin_lock).unlock() }
    }
}

impl<T> Deref for Guard<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*(*self.spin_lock).value.get()) }
    }
}

impl<T> DerefMut for Guard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*(*(self.spin_lock as *mut SpinLock<T>)).value.get_mut()) }
    }
}

pub struct SpinLock<T> {
    value: UnsafeCell<T>,
    owner_ticket: AtomicU64,
    ticket: AtomicU64,
}

unsafe impl<T> Send for SpinLock<T> {}
unsafe impl<T> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        SpinLock {
            value: UnsafeCell::new(value),
            owner_ticket: AtomicU64::new(0),
            ticket: AtomicU64::new(0),
        }
    }
    fn lock(&self) {
        let ticket = self.ticket.fetch_add(1, Ordering::SeqCst);
        while self.owner_ticket.load(Ordering::SeqCst) != ticket {}
    }
    fn unlock(&self) {
        self.owner_ticket.fetch_add(1, Ordering::SeqCst);
    }
    pub fn guard(&self) -> Guard<T> {
        self.lock();
        Guard { spin_lock: self }
    }
}
