use atomic_wait::{wait, wake_all, wake_one};
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, Ordering};

pub struct RwLock<T> {
    state: AtomicU32,
    writer_wake_counter: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for RwLock<T> where T: Send + Sync {}

impl<T> RwLock<T> {
    pub fn new(value: T) -> Self {
        RwLock {
            state: AtomicU32::new(0),
            writer_wake_counter: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    pub fn read(&self) -> ReaderGuard<'_, T> {
        let mut s = self.state.load(Ordering::Relaxed);
        loop {
            if s % 2 == 0 {
                assert!(s < u32::MAX - 1, "too many readers");
                match self.state.compare_exchange_weak(
                    s,
                    s + 2,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => return ReaderGuard { lock: self },
                    Err(e) => s = e,
                }
            }
            if s % 2 == 1 {
                wait(&self.state, u32::MAX);
                s = self.state.load(Ordering::Relaxed);
            }
        }
    }

    pub fn write(&self) -> WriterGuard<'_, T> {
        let mut s = self.state.load(Ordering::Relaxed);
        loop {
            if s <= 1 {
                match self
                    .state
                    .compare_exchange(s, u32::MAX, Ordering::Acquire, Ordering::Relaxed)
                {
                    Ok(_) => return WriterGuard { lock: self },
                    Err(e) => {
                        s = e;
                        continue;
                    }
                }
            }

            if s % 2 == 0 {
                match self
                    .state
                    .compare_exchange(s, s + 1, Ordering::Relaxed, Ordering::Relaxed)
                {
                    Ok(_) => {}
                    Err(e) => {
                        s = e;
                        continue;
                    }
                }
            }

            let w = self.writer_wake_counter.load(Ordering::Acquire);
            s = self.state.load(Ordering::Relaxed);
            if s >= 2 {
                wait(&self.writer_wake_counter, w);
                s = self.state.load(Ordering::Relaxed);
            }
        }
    }
}

pub struct ReaderGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> Deref for ReaderGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> Drop for ReaderGuard<'_, T> {
    fn drop(&mut self) {
        if self.lock.state.fetch_sub(2, Ordering::Release) == 3 {
            self.lock
                .writer_wake_counter
                .fetch_add(1, Ordering::Release);
            wake_one(&self.lock.writer_wake_counter);
        }
    }
}

pub struct WriterGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> Deref for WriterGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for WriterGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for WriterGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.store(0, Ordering::Release);
        self.lock
            .writer_wake_counter
            .fetch_add(1, Ordering::Release);
        wake_one(&self.lock.writer_wake_counter);
        wake_all(&self.lock.state);
    }
}
