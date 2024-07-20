// faster than max flow for most cases
pub struct BipartiteMatching {
    pre: Vec<usize>,
    root: Vec<usize>,
    to: Vec<Vec<usize>>,
    p: Vec<usize>,
    q: Vec<usize>,
    n: usize,
    m: usize,
}

impl BipartiteMatching {
    pub fn new(n: usize, m: usize) -> Self {
        BipartiteMatching {
            pre: vec![usize::MAX; n],
            root: vec![usize::MAX; n],
            to: vec![vec![]; n],
            p: vec![usize::MAX; n],
            q: vec![usize::MAX; m],
            n,
            m,
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        debug_assert!(u < self.n);
        debug_assert!(v < self.m);
        self.to[u].push(v);
    }

    pub fn matching(&mut self) -> usize {
        let mut res = 0;
        let mut updated = true;
        let mut s = std::collections::VecDeque::with_capacity(self.n);
        while updated {
            updated = false;
            for u in 0..self.n {
                if self.p[u] == usize::MAX {
                    self.root[u] = u;
                    s.push_back(u);
                }
            }
            while !s.is_empty() {
                let mut u = s.pop_front().unwrap();
                if self.p[self.root[u]] != usize::MAX {
                    continue;
                }
                for &v in self.to[u].iter() {
                    let mut v = v;
                    if self.q[v] == usize::MAX {
                        while v != usize::MAX {
                            self.q[v] = u;
                            std::mem::swap(&mut self.p[u], &mut v);
                            u = self.pre[u];
                        }
                        updated = true;
                        res += 1;
                        break;
                    }
                    v = self.q[v];
                    if self.pre[v] != usize::MAX {
                        continue;
                    }
                    self.pre[v] = u;
                    self.root[v] = self.root[u];
                    s.push_back(v);
                }
            }
            if updated {
                self.pre.fill(usize::MAX);
                self.root.fill(usize::MAX);
            }
        }
        res
    }
}
