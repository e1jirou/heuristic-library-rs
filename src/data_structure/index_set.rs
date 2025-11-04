#[derive(Debug, Clone)]
pub struct IndexSet {
    n: usize,
    data: Vec<usize>,
    pos: Vec<usize>,
}

impl IndexSet {
    pub fn new(n: usize) -> Self {
        IndexSet {
            n,
            data: Vec::with_capacity(n),
            pos: vec![usize::MAX; n],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, x: usize) -> bool {
        debug_assert!(x < self.n);
        if self.pos[x] == usize::MAX {
            self.pos[x] = self.data.len();
            self.data.push(x);
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self, x: usize) -> bool {
        debug_assert!(x < self.n);
        let i = self.pos[x];
        if i == usize::MAX {
            return false;
        }
        let y = *self.data.last().unwrap();
        self.data[i] = y;
        self.data.pop();
        self.pos[y] = i;
        self.pos[x] = usize::MAX;
        true
    }

    // pop(x) and push(y)
    pub fn replace(&mut self, x: usize, y: usize) {
        debug_assert!(x < self.n);
        debug_assert!(y < self.n);
        debug_assert!(self.contains(x));
        debug_assert!(!self.contains(y));
        if x == y {
            return;
        }
        let i = self.pos[x];
        self.data[i] = y;
        self.pos[x] = usize::MAX;
        self.pos[y] = i;
    }

    pub fn contains(&self, x: usize) -> bool {
        debug_assert!(x < self.n);
        self.pos[x] != usize::MAX
    }

    pub fn clear(&mut self) {
        for &x in &self.data {
            self.pos[x] = usize::MAX;
        }
        self.data.clear();
    }
}
