//! A contiguous growable array written `Vector<T>`.
//!
//! Vectors ensure they never allocate zero sized element and can grow up to `isize::MAX` bytes.

//! The elements of a vector are stored contiguously and can be accessed using offsets. The storage
//! of the vector is handled automatically by expanding the memory as needed and hence a vector
//! data structure takes more memmory as compaired to `std::array`
//!
//! # Examples
//! ```rust
//!# use dsa_sport::datastruct::vec_struct::Vector;
//! let mut v = Vector::new();
//! v.push(1);
//! v.push(2);
//! v.push(3);
//! assert_eq!(format!("{:?}",v), format!("[1][2][3][x]x"));
//! assert_eq!(v.len(), 3);
//! assert_eq!(v.capacity(), 4);
//! ```

use core::mem;
use core::ptr;
use std::alloc;

pub struct Vector<T> {
    pointer: *mut T,
    length: usize,
    capacity: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self {
            pointer: ptr::null_mut() as *mut T,
            length: 0,
            capacity: 0,
        }
    }

    /// Get the raw pointer of primitive array datatype
    ///
    /// # Safety
    /// Do not read the raw pointer if length and capacity is zero
    pub fn as_ptr(&self) -> *mut T {
        return self.pointer;
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Passing the logical index would return an `Option` of the element
    /// # Examples
    /// ```
    ///# use dsa_sport::datastruct::vec_struct::Vector;
    /// let mut v = Vector::new();
    /// v.push('a');
    /// v.push('b');
    /// v.push('c');
    /// assert_eq!(v.get(1), Some(&'b'));
    /// ```
    ///
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        return unsafe { self.pointer.add(index).as_ref() };
    }

    /// Clears the vector from the memorry
    /// ```
    ///# use dsa_sport::datastruct::vec_struct::Vector;
    /// let mut v = Vector::new();
    /// v.push('a');
    /// v.push('b');
    /// v.push('c');
    /// assert_eq!(v.capacity(), 4);
    /// v.clear();
    /// assert_eq!(v.capacity(), 0);
    /// ```
    pub fn clear(&mut self) {
        Vector::deallocate(self);
        self.pointer = ptr::null_mut() as *mut T;
        self.length = 0;
        self.capacity = 0;
    }

    /// write a non zero sized element from the back of the vector,
    ///
    /// # Panics
    /// ```should_panic
    ///# use dsa_sport::datastruct::vec_struct::Vector;
    ///struct Dummy;
    ///let mut v: Vector<Dummy> = Vector::new();
    ///v.push(Dummy);
    /// ```
    ///
    /// # Safety
    /// Declering a Vector doesnot allocate memory in the heap, While initializing the vector it is assume that the capacity of Vector is 4 initially
    /// so to avoid zero sized allocation of memory.
    /// more to see from [`core::alloc::GlobalAlloc::alloc`].
    pub fn push(&mut self, item: T) {
        if std::mem::size_of::<T>() == 0 {
            panic!("Not allowed");
        }
        if self.capacity == 0 {
            let new_capacity = 4;
            let size = mem::size_of::<T>() * new_capacity;
            let align = mem::align_of::<T>();
            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let raw_ptr = alloc::alloc(layout) as *mut T;
                self.pointer = raw_ptr;
                self.pointer.write(item);
            };
            self.length += 1;
            self.capacity = new_capacity;
        } else if self.length < self.capacity {
            let wrap = self
                .length
                .checked_mul(mem::size_of::<T>())
                .expect("cannot reach memory");
            assert!(wrap < std::isize::MAX as usize, "Wrapped isize");
            unsafe {
                self.pointer.add(self.length).write(item);
            }
            self.length += 1;
        } else {
            debug_assert!(self.length == self.capacity);
            let new_capacity = self.capacity.checked_mul(2).expect("capacity wrapped");
            let size = mem::size_of::<T>() * self.capacity;
            let align = mem::align_of::<T>();
            size.checked_add(size % align).expect("cannot allign");
            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = mem::size_of::<T>() * new_capacity;
                let raw_ptr = alloc::realloc(self.pointer as *mut u8, layout, new_size) as *mut T;
                self.pointer = raw_ptr;
                self.pointer.add(self.length).write(item);
            }
            self.length += 1;
            self.capacity = new_capacity;
        }
    }

    fn deallocate(&mut self) {
        unsafe {
            let layout = alloc::Layout::from_size_align_unchecked(
                mem::size_of::<T>() * self.capacity,
                mem::align_of::<T>(),
            );
            alloc::dealloc(self.pointer as *mut u8, layout)
        }
    }
}

impl<T> std::fmt::Debug for Vector<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        if self.pointer.is_null() {
            out += "x";
        } else {
            for i in 0..self.length {
                unsafe {
                    out += &format!("[{:?}]", self.pointer.add(i).read());
                }
            }
            for _ in 0..self.capacity - self.length {
                out += "[x]";
            }
            out += "x";
        }
        write!(f, "{}", out)
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        Vector::deallocate(self);
    }
}

#[cfg(test)]
mod tedts {
    use super::*;

    #[test]
    #[should_panic]
    fn zero_size_type() {
        struct Dummy;
        let mut v: Vector<Dummy> = Vector::new();
        v.push(Dummy);
    }
}
