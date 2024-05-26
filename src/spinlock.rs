use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering::*;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Acquire) {
            core::hint::spin_loop();
        }
    }

    /// # Safety
    /// &mut T which lock() returns should be disappeared.
    /// (You shouldn't hold reference to T!)
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // safety:
        // This guard's existence guarantees we acqire lock exclusively
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // safety:
        // This guard's existence guarantees we acqire lock exclusively
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        unsafe { self.lock.unlock() };
    }
}

unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}
