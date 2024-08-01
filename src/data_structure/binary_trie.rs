struct Node {
    children: [usize; 2],
    size: usize,  // subtree size
}

// multiset
pub struct BinaryTrie {
    log: usize,  // tree height
    nodes: Vec<Node>,
    pool: Vec<usize>,
}

impl BinaryTrie {
    pub fn new(log: usize) -> Self {
        debug_assert!(log <= 64);
        let root = Node {
            children: [usize::MAX, usize::MAX],
            size: 0,
        };
        BinaryTrie {
            log,
            nodes: vec![root],
            pool: Vec::new(),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }

    pub fn is_empty(&self) -> bool {
        self.nodes[0].size == 0
    }

    pub fn len(&self) -> usize {
        self.nodes[0].size
    }

    fn make_node(&mut self, size: usize) -> usize {
        match self.pool.pop() {
            Some(v) => {
                self.nodes[v] = Node {
                    children: [usize::MAX, usize::MAX],
                    size,
                };
                v
            }
            None => {
                let v = self.nodes.len();
                self.nodes.push(Node {
                    children: [usize::MAX, usize::MAX],
                    size,
                });
                v
            }
        }
    }

    pub fn count(&self, x: u64) -> usize {
        debug_assert!(self.log == 64 || x < (1 << self.log));
        let mut v = 0;
        for depth in (0..self.log).rev() {
            v = self.nodes[v].children[((x >> depth) & 1) as usize];
            if v == usize::MAX {
                return 0;
            }
        }
        self.nodes[v].size
    }

    pub fn insert(&mut self, x: u64, n: usize) {
        debug_assert!(self.log == 64 || x < (1 << self.log));
        let mut parent = 0;
        self.nodes[parent].size += n;
        for depth in (0..self.log).rev() {
            let left_or_right = ((x >> depth) & 1) as usize;
            let mut child = self.nodes[parent].children[left_or_right];
            if child == usize::MAX {
                child = self.make_node(n);
                self.nodes[parent].children[left_or_right] = child;
            } else {
                self.nodes[child].size += n;
            }
            parent = child;
        }
    }

    // return the number of erased items
    pub fn erase(&mut self, x: u64, mut n: usize) -> usize {
        debug_assert!(self.log == 64 || x < (1 << self.log));
        let mut v = 0;
        for depth in (0..self.log).rev() {
            v = self.nodes[v].children[((x >> depth) & 1) as usize];
            if v == usize::MAX {
                return 0;
            }
        }
        n = n.min(self.nodes[v].size);

        self.nodes[0].size -= n;
        let mut parent = 0;
        for depth in (0..self.log).rev() {
            let left_or_right = ((x >> depth) & 1) as usize;
            let child = self.nodes[parent].children[left_or_right];
            self.nodes[child].size -= n;
            if self.nodes[child].size == 0 {
                self.nodes[parent].children[left_or_right] = usize::MAX;
                self.pool.push(child);
            }
            parent = child;
        }
        n
    }

    pub fn min(&self) -> Option<u64> {
        if self.is_empty() {
            return None;
        }
        let mut parent = 0;
        let mut ret = 0;
        for depth in (0..self.log).rev() {
            let child = self.nodes[parent].children[0];
            if child == usize::MAX {
                parent = self.nodes[parent].children[1];
                ret |= 1 << depth;
            } else {
                parent = child;
            }
        }
        Some(ret)
    }

    pub fn max(&self) -> Option<u64> {
        if self.is_empty() {
            return None;
        }
        let mut parent = 0;
        let mut ret = 0;
        for depth in (0..self.log).rev() {
            let child = self.nodes[parent].children[1];
            if child == usize::MAX {
                parent = self.nodes[parent].children[0];
            } else {
                parent = child;
                ret |= 1 << depth;
            }
        }
        Some(ret)
    }

    // return min before xor (argmin)
    pub fn xor_min(&self, x: u64) -> Option<u64> {
        debug_assert!(self.log == 64 || x < (1 << self.log));
        if self.is_empty() {
            return None;
        }
        let mut parent = 0;
        let mut ret = 0;
        for depth in (0..self.log).rev() {
            if ((x >> depth) & 1) == 0 {
                let child = self.nodes[parent].children[0];
                if child == usize::MAX {
                    parent = self.nodes[parent].children[1];
                    ret |= 1 << depth;
                } else {
                    parent = child;
                }
            } else {
                let child = self.nodes[parent].children[1];
                if child == usize::MAX {
                    parent = self.nodes[parent].children[0];
                } else {
                    parent = child;
                    ret |= 1 << depth;
                }
            }
        }
        Some(ret)
    }

    // return max before xor (argmax)
    pub fn xor_max(&self, x: u64) -> Option<u64> {
        debug_assert!(self.log == 64 || x < (1 << self.log));
        if self.is_empty() {
            return None;
        }
        let mut parent = 0;
        let mut ret = 0;
        for depth in (0..self.log).rev() {
            if ((x >> depth) & 1) == 1 {
                let child = self.nodes[parent].children[0];
                if child == usize::MAX {
                    parent = self.nodes[parent].children[1];
                    ret |= 1 << depth;
                } else {
                    parent = child;
                }
            } else {
                let child = self.nodes[parent].children[1];
                if child == usize::MAX {
                    parent = self.nodes[parent].children[0];
                } else {
                    parent = child;
                    ret |= 1 << depth;
                }
            }
        }
        Some(ret)
    }

    pub fn get_kth(&self, mut k: usize) -> Option<u64> {
        if k >= self.len() {
            return None;
        }
        let mut parent = 0;
        let mut x = 0;
        for depth in (0..self.log).rev() {
            let mut child = self.nodes[parent].children[0];
            if child == usize::MAX {
                x |= 1 << depth;
                child = self.nodes[parent].children[1];
            } else if self.nodes[child].size <= k {
                k -= self.nodes[child].size;
                x |= 1 << depth;
                child = self.nodes[parent].children[1];
            }
            parent = child;
        }
        Some(x)
    }

    // count items less than x
    pub fn bisect(&self, x: u64) -> usize {
        debug_assert!(self.log == 64 || x < (1 << self.log));
        let mut v = 0;
        let mut cnt = 0;
        for depth in (0..self.log).rev() {
            let left_or_right = ((x >> depth) & 1) as usize;
            if left_or_right == 1 {
                let u = self.nodes[v].children[0];
                if u != usize::MAX {
                    cnt += self.nodes[u].size;
                }
            }
            v = self.nodes[v].children[left_or_right];
            if v == usize::MAX {
                return cnt;
            }
        }
        cnt
    }
}
