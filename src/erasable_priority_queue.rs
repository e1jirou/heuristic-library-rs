use std::collections::BinaryHeap;

pub struct ErasablePriorityQueue<T> {
    data: BinaryHeap<T>,
    lazy: BinaryHeap<T>,
}

impl<T: Ord> ErasablePriorityQueue<T> {
    pub fn new() -> Self {
        ErasablePriorityQueue {
            data: BinaryHeap::new(),
            lazy: BinaryHeap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ErasablePriorityQueue {
            data: BinaryHeap::with_capacity(capacity),
            lazy: BinaryHeap::with_capacity(capacity),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn erase(&mut self, item: T) {
        if *self.data.peek().unwrap() == item {
            self.data.pop();
            self.update();
        } else {
            self.lazy.push(item);
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.peek()
    }

    fn update(&mut self) {
        while !self.lazy.is_empty() && self.data.peek().unwrap() == self.lazy.peek().unwrap() {
            self.data.pop();
            self.lazy.pop();
        }
    }
}
