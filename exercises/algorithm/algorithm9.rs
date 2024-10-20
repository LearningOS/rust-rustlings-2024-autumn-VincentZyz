/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;
use std::mem;

pub struct Heap<T>
where
    T: Default + Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);
        self.bubble_up(self.count);
    }

    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if !self.children_present(idx) {
            return 0;
        }

        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        if right_idx > self.count || (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }

    fn bubble_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let child_idx = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                self.items.swap(idx, child_idx);
                idx = child_idx;
            } else {
                break;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            // Use split_at_mut to avoid simultaneous mutable and immutable borrows
            let (first, rest) = self.items.split_at_mut(self.count);
            let root = mem::replace(&mut first[1], rest[rest.len() - 1].clone());
            self.count -= 1;
            self.items.pop();
            if !self.is_empty() {
                self.bubble_down(1);
            }
            Some(root)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a > b)
    }
}


// impl<T> Heap<T>
// where
//     T: Default,
// {
//     pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
//         Self {
//             count: 0,
//             items: vec![T::default()],
//             comparator,
//         }
//     }

//     pub fn len(&self) -> usize {
//         self.count
//     }

//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }

//     pub fn add(&mut self, value: T) {
//         //TODO
//     }

//     fn parent_idx(&self, idx: usize) -> usize {
//         idx / 2
//     }

//     fn children_present(&self, idx: usize) -> bool {
//         self.left_child_idx(idx) <= self.count
//     }

//     fn left_child_idx(&self, idx: usize) -> usize {
//         idx * 2
//     }

//     fn right_child_idx(&self, idx: usize) -> usize {
//         self.left_child_idx(idx) + 1
//     }

//     fn smallest_child_idx(&self, idx: usize) -> usize {
//         //TODO
// 		0
//     }
// }

// impl<T> Heap<T>
// where
//     T: Default + Ord,
// {
//     /// Create a new MinHeap
//     pub fn new_min() -> Self {
//         Self::new(|a, b| a < b)
//     }

//     /// Create a new MaxHeap
//     pub fn new_max() -> Self {
//         Self::new(|a, b| a > b)
//     }
// }

// impl<T> Iterator for Heap<T>
// where
//     T: Default,
// {
//     type Item = T;

//     fn next(&mut self) -> Option<T> {
//         //TODO
// 		None
//     }
// }

// pub struct MinHeap;

// impl MinHeap {
//     #[allow(clippy::new_ret_no_self)]
//     pub fn new<T>() -> Heap<T>
//     where
//         T: Default + Ord,
//     {
//         Heap::new(|a, b| a < b)
//     }
// }

// pub struct MaxHeap;

// impl MaxHeap {
//     #[allow(clippy::new_ret_no_self)]
//     pub fn new<T>() -> Heap<T>
//     where
//         T: Default + Ord,
//     {
//         Heap::new(|a, b| a > b)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}