//! # sort_analysis
//!
//! `sort_analysis` crate really not usefull in online Judge platform but this crate will ensure
//! that you have all the materials that you need to learn about sorting algorithm complexities
/// Bubble Sort algorithm in its most basic form
/// 
/// # Example
///
/// ```
/// let mut v = vec![4, 6, 1, 8, 11, 13, 3];
/// dsa_sport::sort_analysis::bub_sort::bubble_sort(&mut v);
/// assert_eq!(v, vec![1,3,4,6,8,11,13])
///```
pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    /*! O(n^2) !*/
    for _pass in 0..v.len() {
        for i in 0..v.len()-1 {
            if v[i] > v[i + 1] {
                v.swap(i, i+1);
            }
        }
    }
}


pub fn bubble_sort_improved<T: PartialOrd>(v: &mut [T]) {
    for pass in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len()-1)-pass {
            if v[i] > v[i + 1] {
                v.swap(i, i+1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::sort_analysis::bub_sort::{bubble_sort, bubble_sort_improved};

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1,3,4,6,8,11,13])
    }

    #[test]
    fn test_bubble_sort_improved() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort_improved(&mut v);
        assert_eq!(v, vec![1,3,4,6,8,11,13])
    }
}
