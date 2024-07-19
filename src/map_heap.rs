// max heap
pub struct MapHeap<T> {
    n: usize,
    heap: Vec<(usize, T)>, // 1-indexed
    indices: Vec<usize>,
}

impl<T: Clone + Default + Ord> MapHeap<T> {
    pub fn new(n: usize) -> Self {
        let mut heap = vec![(usize::MAX, T::default())];
        heap.reserve(n);
        MapHeap {
            n,
            heap,
            indices: vec![usize::MAX; n],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.len() <= 1
    }

    pub fn insert(&mut self, p: usize, x: T) {
        debug_assert!(p < self.n);
        let i = self.indices[p];
        if i == usize::MAX {
            // push
            let i = self.heap.len();
            self.heap.push((usize::MAX, T::default()));
            self.up(i, p, x);
        } else if self.heap[i].1 < x {
            // increase
            self.up(i, p, x);
        } else {
            // decrease
            self.down(i, p, x);
        }
    }

    pub fn remove(&mut self, p: usize) -> Option<T> {
        debug_assert!(p < self.n);
        let i = self.indices[p];
        if i == usize::MAX {
            return None;
        }
        let ret = self.heap[i].1.clone();
        self.indices[p] = usize::MAX;
        if self.heap.len() == 2 {
            self.heap.pop();
            return Some(ret);
        }
        let (q, y) = self.heap.pop().unwrap();
        self.down(i, q, y);
        Some(ret)
    }

    pub fn peek(&self) -> Option<&(usize, T)> {
        if self.is_empty() {
            None
        } else {
            Some(&self.heap[1])
        }
    }

    pub fn pop(&mut self) -> Option<(usize, T)> {
        if self.is_empty() {
            return None;
        }
        if self.heap.len() == 2 {
            return self.heap.pop()
        }
        let ret = self.heap[1].clone();
        self.indices[ret.0] = usize::MAX;
        let (p, x) = self.heap.pop().unwrap();
        self.down(1, p, x);
        Some(ret)
    }

    fn up(&mut self, mut i: usize, p: usize, x: T) {
        while i > 1 {
            let (q, y) = self.heap[i >> 1].clone();
            if x <= y {
                self.indices[p] = i;
                self.heap[i] = (p, x);
                return;
            }
            // move up
            self.indices[q] = i;
            self.heap[i] = (q, y);
            i >>= 1;
        }
        self.indices[p] = 1;
        self.heap[1] = (p, x);
    }

    fn down(&mut self, mut i: usize, p: usize, x: T) {
        while (i << 1) < self.heap.len() {
            let j = i << 1;
            let (q, y) = self.heap[j].clone();
            if j + 1 == self.heap.len() {
                if y <= x {
                    self.indices[p] = i;
                    self.heap[i] = (p, x);
                    return;
                }
                // move down
                self.indices[q] = i;
                self.heap[i] = (q, y);
                i = j;
                break;
            }
            let (r, z) = self.heap[j + 1].clone();
            if z <= y {
                if y <= x {
                    self.indices[p] = i;
                    self.heap[i] = (p, x);
                    return;
                }
                // move down
                self.indices[q] = i;
                self.heap[i] = (q, y);
                i = j;
            } else {
                if z <= x {
                    self.indices[p] = i;
                    self.heap[i] = (p, x);
                    return;
                }
                // move down
                self.indices[r] = i;
                self.heap[i] = (r, z);
                i = j + 1;
            }
        }
        self.indices[p] = i;
        self.heap[i] = (p, x);
    }
}
