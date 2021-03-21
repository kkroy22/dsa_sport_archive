//! LIFO operated container adaptor `VecStack<T>` and `ListStack`.
//!
//! Stacks are a type of container adaptor, specifically designed to operate in a LIFO context (last-in first-out), where elements are inserted and extracted only from one end of the container.
//! # Examples
//! ```
//! # use dsa_sport::datastruct::stack_struct::VecStack;
//! let mut vs = VecStack::new();
//! vs.push(1);
//! vs.push(2);
//! vs.push(3);
//! assert_eq!(format!("{:?}",vs), format!("[1|2|3"));
//! assert_eq!(vs.size(), 3);
//! assert_eq!(vs.capacity(), 4);
//! ```

use crate::datastruct::vec_struct::Vector;

pub struct VecStack<T> {
    sk_ptr: Vector<T>,
    sk_index: usize,
}

impl<T> VecStack<T> {
    pub fn new() -> Self {
        return Self {
            sk_ptr: Vector::new(),
            sk_index: 0,
        };
    }

    pub fn size(&self) -> usize {
        return self.sk_index;
    }

    pub fn capacity(&self) -> usize {
        return self.sk_ptr.capacity();
    }

    pub fn is_empty(&self) -> bool {
        return self.sk_index == 0;
    }

    pub fn top(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        let ans = self.sk_ptr.get(self.sk_index -1);
        return Some(ans.unwrap());
    }

    pub fn push(&mut self, element: T) {
        self.sk_ptr.push(element);
        self.sk_index += 1;
    }

    pub fn pop(&mut self) -> Option<&T>
    {
        if self.is_empty() {
            return None;
        } else {
            self.sk_index -= 1;
            let ans = self.sk_ptr.get(self.sk_index);
            return Some(ans.unwrap());
        }
    }
}

impl<T> std::fmt::Debug for VecStack<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        if self.sk_index == 0 {
            out += "[x";
        } else {
            for i in 0..self.sk_index {
                if i == 0 {
                    out += &format!("[{:?}", self.sk_ptr.get(i).unwrap());
                } else {
                    out += &format!("|{:?}", self.sk_ptr.get(i).unwrap());
                }
            }
        }
        return write!(f, "{}", out);
    }
}
