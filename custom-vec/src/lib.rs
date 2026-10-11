use std::alloc::Layout;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::{alloc, ptr};

pub struct Vec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        assert_ne!(size_of::<T>(), 0, "zero sized types are not supported");
        Vec {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow()
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), value);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;
            (new_cap, Layout::array::<T>(new_cap).unwrap())
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation exceeded"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { alloc::realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr.as_ptr().add(index), elem);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr.as_ptr().add(index));
            ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index,
            );
            result
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) };
        }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}
