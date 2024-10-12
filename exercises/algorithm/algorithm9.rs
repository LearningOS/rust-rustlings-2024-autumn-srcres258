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
    pub items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![],
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
        // println!("Adding");

        // Check the capacity of Vec at first before pushing the new value.
        let cap = self.items.capacity();
        if self.count >= cap {
            // If the Vec object is full, enlarge its capacity.
            let count_exp = (cap as f32).log2();
            // Put the capacity to the next exponential level based on 2.
            let new_cap = (count_exp + 1.0).floor().exp2() as usize;
            self.items.resize_with(new_cap, T::default);
        }

        // Then add the new value into Vec.
        self.items.push(value);
        self.count += 1;

        // Rebuild the heap so that it meets the requirement provided
        // by the comparator.
        if self.count > 1 {
            let mut cur_idx = self.count - 1;

            // println!("cur_idx is {}", cur_idx);
            while cur_idx > 0 {
                let parent_idx = self.parent_idx(cur_idx);
                // println!("parent_idx is {}", parent_idx);

                let cur_elem = self.items.get(cur_idx).unwrap();
                let parent_elem = self.items.get(parent_idx).unwrap();
                let satisfied = (self.comparator)(parent_elem, cur_elem);
                // let satisfied = (self.comparator)(cur_elem, parent_elem);
                if satisfied {
                    // println!("Satisfied!");
                    break;
                }

                // If not satisfied, swap the current node with the parent
                // node.
                // println!("Swapping {} and {}", cur_idx, parent_idx);
                self.items.swap(cur_idx, parent_idx);

                // Step onto higher layer of the heap.
                cur_idx = parent_idx;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 2
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
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
        // println!("Getting next");

        match self.count {
            0 => None,
            1 => {
                let result = Some(self.items.pop().unwrap());
                self.count -= 1;
                result
            }
            _ => {
                self.items.swap(0, self.count - 1);
                let result = Some(self.items.pop().unwrap());
                self.count -= 1;

                let mut cur_idx = 0;
                while self.children_present(cur_idx) {
                    let left_child_idx = self.left_child_idx(cur_idx);
                    let right_child_idx = self.right_child_idx(cur_idx);
                    let left_child_presents = left_child_idx < self.count;
                    let right_child_presents = right_child_idx < self.count;
                    let mut should_break = true;

                    let cur_elem = self.items.get(cur_idx).unwrap();
                    // println!("cur_idx is {}", cur_idx);
                    if left_child_presents {
                        // println!("left_child_idx is {}", left_child_idx);
                        let left_child_elem = self.items.get(left_child_idx).unwrap();
                        if !(self.comparator)(cur_elem, left_child_elem) {
                            // println!("Swapping {} with left child {}", cur_idx, left_child_idx);
                            self.items.swap(cur_idx, left_child_idx);
                            cur_idx = left_child_idx;
                            should_break = false;
                        }
                    } else if right_child_presents {
                        // println!("right_child_idx is {}", right_child_idx);
                        let right_child_elem = self.items.get(right_child_idx).unwrap();
                        if !(self.comparator)(cur_elem, right_child_elem) {
                            // println!("Swapping {} with right child {}", cur_idx, right_child_idx);
                            self.items.swap(cur_idx, right_child_idx);
                            cur_idx = right_child_idx;
                            should_break = false;
                        }
                    }

                    if should_break {
                        break;
                    }
                }

                result
            }
        }
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
        // println!("After added: {:?}", heap.items);
        heap.add(2);
        // println!("After added: {:?}", heap.items);
        heap.add(9);
        // println!("After added: {:?}", heap.items);
        heap.add(11);
        // println!("After added: {:?}", heap.items);
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