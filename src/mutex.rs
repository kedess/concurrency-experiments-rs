use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{
        AtomicBool, AtomicU64,
        Ordering::{Acquire, Relaxed, Release},
    },
};

#[derive(Clone)]
pub struct GuardTicket<T> {
    spin_lock: *const SpinLockTicket<T>,
}

impl<T> Drop for GuardTicket<T> {
    fn drop(&mut self) {
        unsafe { (*self.spin_lock).unlock() }
    }
}

impl<T> Deref for GuardTicket<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &(*(*self.spin_lock).value.get()) }
    }
}

impl<T> DerefMut for GuardTicket<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut (*(*(self.spin_lock as *mut SpinLockTicket<T>))
                .value
                .get_mut())
        }
    }
}

pub struct SpinLockTicket<T> {
    value: UnsafeCell<T>,
    owner_ticket: AtomicU64,
    ticket: AtomicU64,
}

unsafe impl<T> Send for SpinLockTicket<T> {}
unsafe impl<T> Sync for SpinLockTicket<T> {}

impl<T> SpinLockTicket<T> {
    pub fn new(value: T) -> Self {
        SpinLockTicket {
            value: UnsafeCell::new(value),
            owner_ticket: AtomicU64::new(0),
            ticket: AtomicU64::new(0),
        }
    }
    fn lock(&self) {
        let ticket = self.ticket.fetch_add(1, Relaxed);
        while self.owner_ticket.load(Acquire) != ticket {}
    }
    fn unlock(&self) {
        self.owner_ticket.fetch_add(1, Release);
    }
    pub fn guard(&self) -> GuardTicket<T> {
        self.lock();
        GuardTicket { spin_lock: self }
    }
}

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
    lock: AtomicBool,
}

unsafe impl<T> Send for SpinLock<T> {}
unsafe impl<T> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        SpinLock {
            value: UnsafeCell::new(value),
            lock: AtomicBool::new(false),
        }
    }
    fn lock(&self) {
        while self.lock.compare_exchange_weak(false, true, Acquire, Relaxed).is_err() {
            while self.lock.load(Relaxed) {
                std::hint::spin_loop();
            }
        }
    }
    fn unlock(&self) {
        self.lock.store(false, Release);
    }
    pub fn guard(&self) -> Guard<T> {
        self.lock();
        Guard { spin_lock: self }
    }
}