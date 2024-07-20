// Double-Ended Priority Queue
pub struct IntervalHeap<T> {
    heap: Vec<T>,
}

impl<T: Ord> IntervalHeap<T> {
    pub fn new() -> Self {
        IntervalHeap {
            heap: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        IntervalHeap {
            heap: Vec::with_capacity(capacity),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.heap.reserve(additional);
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn push(&mut self, x: T) {
        let p = self.heap.len();
        self.heap.push(x);
        self.up(p);
    }

    pub fn peek_min(&self) -> Option<&T> {
        if self.heap.len() < 2 {
            self.heap.last()
        } else {
            Some(&self.heap[1])
        }
    }

    pub fn peek_max(&self) -> Option<&T> {
        if self.heap.is_empty() {
            None
        } else {
            Some(&self.heap[0])
        }
    }

    pub fn push_pop_min(&mut self, mut x: T) -> T {
        if self.heap.len() == 0 {
            return x;
        }
        if self.heap.len() == 1 {
            if self.heap[0] < x {
                std::mem::swap(&mut self.heap[0], &mut x);
            }
            return x;
        }
        if x <= self.heap[1] {
            return x;
        }
        std::mem::swap(&mut self.heap[1], &mut x);
        let p = self.down(1);
        self.up(p);
        x
    }

    pub fn push_pop_max(&mut self, mut x: T) -> T {
        if self.is_empty() {
            return x;
        }
        if x >= self.heap[0] {
            return x;
        }
        std::mem::swap(&mut self.heap[0], &mut x);
        let p = self.down(0);
        self.up(p);
        x
    }

    pub fn pop_min(&mut self) -> Option<T> {
        if self.heap.len() < 3 {
            self.heap.pop()
        } else {
            let n = self.heap.len();
            self.heap.swap(1, n - 1);
            let ret = self.heap.pop();
            let p = self.down(1);
            self.up(p);
            ret
        }
    }

    pub fn pop_max(&mut self) -> Option<T> {
        if self.heap.len() < 2 {
            self.heap.pop()
        } else {
            let n = self.heap.len();
            self.heap.swap(0, n - 1);
            let ret = self.heap.pop();
            let p = self.down(0);
            self.up(p);
            ret
        }
    }

    fn parent(p: usize) -> usize {
        ((p >> 1) - 1) & !1
    }

    fn up(&mut self, mut p: usize) -> usize {
        if p | 1 < self.heap.len() && self.heap[p & !1] < self.heap[p | 1] {
            self.heap.swap(p & !1, p | 1);
            p ^= 1;
        }
        // max heap
        while p > 1 {
            let q = Self::parent(p);
            if self.heap[p] <= self.heap[q] {
                break;
            }
            self.heap.swap(p, q);
            p = q;
        }
        // min heap
        while p > 1 {
            let q = Self::parent(p) | 1;
            if self.heap[q] <= self.heap[p] {
                break;
            }
            self.heap.swap(p, q);
            p = q;
        }
        p
    }

    fn down(&mut self, mut p: usize) -> usize {
        let n = self.heap.len();
        if (p & 1) == 1 {
            // min heap
            while 2 * p + 1 < n {
                let mut q = 2 * p + 3;
                if n <= q || self.heap[q - 2] < self.heap[q] {
                    q -= 2;
                }
                if q < n && self.heap[q] < self.heap[p] {
                    self.heap.swap(p, q);
                    p = q;
                } else {
                    break;
                }
            }
        } else {
            // max heap
            while 2 * p + 2 < n {
                let mut q = 2 * p + 4;
                if n <= q || self.heap[q] < self.heap[q - 2] {
                    q -= 2
                }
                if q < n && self.heap[p] < self.heap[q] {
                    self.heap.swap(p, q);
                    p = q;
                } else {
                    break;
                }
            }
        }
        p
    }
}
