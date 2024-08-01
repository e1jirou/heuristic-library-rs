pub struct DisjointSetUnion {
    n: usize,
    parent_or_size: Vec<i32>,
}

impl DisjointSetUnion {
    pub fn new(n: usize) -> Self {
        DisjointSetUnion {
            n,
            parent_or_size: vec![-1; n],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        debug_assert!(a < self.n);
        debug_assert!(b < self.n);
        let mut x = self.leader(a);
        let mut y = self.leader(b);
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        debug_assert!(a < self.n);
        debug_assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }

    pub fn leader(&mut self, a: usize) -> usize {
        debug_assert!(a < self.n);
        if self.parent_or_size[a] < 0 {
            return a;
        }
        let root = self.leader(self.parent_or_size[a] as usize);
        self.parent_or_size[a] = root as i32;
        root
    }

    pub fn size(&mut self, a: usize) -> usize {
        let x = self.leader(a);
        (-self.parent_or_size[x]) as usize
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![vec![]; self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result.retain(|v| !v.is_empty());
        result
    }
}

pub struct UndoableDisjointSetUnion {
    n: usize,
    parent_or_size: Vec<i32>,
    history: Vec<(usize, i32, usize, i32)>,
}

impl UndoableDisjointSetUnion {
    pub fn new(n: usize) -> Self {
        UndoableDisjointSetUnion {
            n,
            parent_or_size: vec![-1; n],
            history: Vec::with_capacity(n),
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        debug_assert!(a < self.n);
        debug_assert!(b < self.n);
        let mut x = self.leader(a);
        let mut y = self.leader(b);
        self.history.push((x, self.parent_or_size[x], y, self.parent_or_size[y]));
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        debug_assert!(a < self.n);
        debug_assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }

    pub fn leader(&mut self, a: usize) -> usize {
        debug_assert!(a < self.n);
        if self.parent_or_size[a] < 0 {
            return a;
        }
        self.leader(self.parent_or_size[a] as usize)
    }

    pub fn size(&mut self, a: usize) -> usize {
        let x = self.leader(a);
        (-self.parent_or_size[x]) as usize
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![vec![]; self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result.retain(|v| !v.is_empty());
        result
    }

    pub fn undo(&mut self) -> (usize, usize) {
        debug_assert!(!self.history.is_empty());
        let (a, x, b, y) = *self.history.last().unwrap();
        self.history.pop();
        self.parent_or_size[a] = x;
        self.parent_or_size[b] = y;
        (a, b)
    }
}

pub struct DisjointSetUnionWithMonoid<M: Monoid> {
    n: usize,
    parent_or_size: Vec<i32>,
    data: Vec<M::S>,
}

impl<M: Monoid> DisjointSetUnionWithMonoid<M> {
    pub fn new(n: usize) -> Self {
        DisjointSetUnionWithMonoid {
            n,
            parent_or_size: vec![-1; n],
            data: vec![M::e(); n],
        }
    }

    pub fn from_slice(v: &[M::S]) -> Self {
        DisjointSetUnionWithMonoid {
            n: v.len(),
            parent_or_size: vec![-1; v.len()],
            data: v.to_vec(),
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        debug_assert!(a < self.n);
        debug_assert!(b < self.n);
        let mut x = self.leader(a);
        let mut y = self.leader(b);
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        self.data[x] = M::op(&self.data[x], &self.data[y]);
        x
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        debug_assert!(a < self.n);
        debug_assert!(b < self.n);
        self.leader(a) == self.leader(b)
    }

    pub fn leader(&mut self, a: usize) -> usize {
        debug_assert!(a < self.n);
        if self.parent_or_size[a] < 0 {
            return a;
        }
        let root = self.leader(self.parent_or_size[a] as usize);
        self.parent_or_size[a] = root as i32;
        root
    }

    pub fn size(&mut self, a: usize) -> usize {
        let x = self.leader(a);
        (-self.parent_or_size[x]) as usize
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            leader_buf[i] = self.leader(i);
            group_size[leader_buf[i]] += 1;
        }
        let mut result = vec![vec![]; self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[leader_buf[i]].push(i);
        }
        result.retain(|v| !v.is_empty());
        result
    }

    pub fn get(&mut self, a: usize) -> &M::S {
        let root = self.leader(a);
        &self.data[root]
    }

    pub fn set(&mut self, a: usize, x: M::S) {
        let root = self.leader(a);
        self.data[root] = x;
    }

    pub fn apply(&mut self, a: usize, x: &M::S) {
        let root = self.leader(a);
        self.data[root] = M::op(&self.data[root], &x);
    }
}

pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

// struct MinMonoid;

// impl Monoid for MinMonoid {
//     type S = i32;

//     fn e() -> Self::S {
//         Self::S::MAX
//     }

//     fn op(a: &Self::S, b: &Self::S) -> Self::S {
//         *a.min(b)
//     }
// }
