pub struct ErasablePriorityQueue<T> {
    data: std::collections::BinaryHeap<T>,
    lazy: std::collections::BinaryHeap<T>,
}

impl<T: Ord> ErasablePriorityQueue<T> {
    pub fn new() -> Self {
        ErasablePriorityQueue {
            data: std::collections::BinaryHeap::new(),
            lazy: std::collections::BinaryHeap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ErasablePriorityQueue {
            data: std::collections::BinaryHeap::with_capacity(capacity),
            lazy: std::collections::BinaryHeap::with_capacity(capacity),
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

    pub fn len(&self) -> usize {
        self.data.len() - self.lazy.len()
    }

    fn update(&mut self) {
        while !self.lazy.is_empty() && self.data.peek().unwrap() == self.lazy.peek().unwrap() {
            self.data.pop();
            self.lazy.pop();
        }
    }
}

pub struct DoubleEndedPriorityQueue<T> {
    min_queue: ErasablePriorityQueue<std::cmp::Reverse<T>>,
    max_queue: ErasablePriorityQueue<T>,
}

impl <T: Clone + Ord> DoubleEndedPriorityQueue<T> {
    pub fn new() -> Self {
        DoubleEndedPriorityQueue {
            min_queue: ErasablePriorityQueue::new(),
            max_queue: ErasablePriorityQueue::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DoubleEndedPriorityQueue {
            min_queue: ErasablePriorityQueue::with_capacity(capacity),
            max_queue: ErasablePriorityQueue::with_capacity(capacity),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min_queue.is_empty()
    }

    pub fn push(&mut self, item: T) {
        self.min_queue.push(std::cmp::Reverse(item.clone()));
        self.max_queue.push(item);
    }

    pub fn erase(&mut self, item: T) {
        self.min_queue.erase(std::cmp::Reverse(item.clone()));
        self.max_queue.erase(item);
    }

    pub fn peek_min(&self) -> Option<&T> {
        match self.min_queue.peek() {
            Some(ret) => Some(&ret.0),
            None => None,
        }
    }

    pub fn peek_max(&self) -> Option<&T> {
        self.max_queue.peek()
    }

    pub fn len(&self) -> usize {
        self.min_queue.len()
    }
}
