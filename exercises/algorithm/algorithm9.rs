/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
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
        //TODO
        self.count += 1;
        if self.count == self.items.len() {
            self.items.push(T::default());
        }

        let mut idx = self.count;
        self.items[idx] = value;

        while idx > 1 {
            let p_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[p_idx]) {
                self.items.swap(idx, p_idx);
                idx = p_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn child_present(&self, idx: usize) -> bool {
        self.has_left_child(idx)
    }
    
    fn has_left_child(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn has_right_child(&self, idx: usize) -> bool {
        self.right_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> Option<usize> {
        //TODO
        if !self.child_present(idx)  {
            return None;
        }

        let l_idx = self.left_child_idx(idx);
        let r_idx = if self.has_right_child(idx) {
            Some(self.right_child_idx(idx))
        } else {
            None
        };

        match r_idx {
            None => Some(l_idx),
            Some(idx) => {
                if (self.comparator)(&self.items[l_idx], &self.items[idx]) {
                    Some(l_idx)
                } else {
                    Some(idx)
                }
            }
        }
    }

    fn bubble_down(&mut self, idx: usize) {
        let mut cur_idx = idx;
        while let Some(child_idx) = self.smallest_child_idx(cur_idx) {
            if (self.comparator)(&self.items[cur_idx], &self.items[child_idx]) {
                break;
            } else {
                self.items.swap(cur_idx, child_idx);
                cur_idx = child_idx;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.count > 0 {
            self.items.swap(1, self.count);
            let ret = self.items.pop();
            self.count -= 1;
            self.bubble_down(1);
            return ret;
        }
        None
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

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