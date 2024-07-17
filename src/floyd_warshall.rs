pub struct FloydWarshall<T> {
    n: usize,
    edges: Vec<T>,
    costs: Vec<T>,
    parents: Vec<usize>,
}

impl<T: num_traits::NumAssign + num_traits::PrimInt> FloydWarshall<T> {
    pub fn new(n: usize) -> Self {
        let inf = T::max_value() >> 1;
        FloydWarshall {
            n,
            edges: vec![inf; n * n],
            costs: vec![T::zero(); n * n],
            parents: vec![0; n * n],
        }
    }

    pub fn add_directed_edge(&mut self, u: usize, v: usize, c: T) {
        debug_assert!(u < self.n);
        debug_assert!(v < self.n);
        let i = self.n * u + v;
        self.edges[i] = self.edges[i].min(c);
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize, c: T) {
        debug_assert!(u < self.n);
        debug_assert!(v < self.n);
        self.add_directed_edge(u, v, c);
        self.add_directed_edge(v, u, c);
    }

    pub fn floyd_warshall(&mut self) {
        self.costs.copy_from_slice(&self.edges);
        for k in 0..self.n {
            let nk = self.n * k;
            for i in 0..self.n {
                let ni = self.n * i;
                for j in 0..self.n {
                    let c = self.costs[ni + k] + self.costs[nk + j];
                    let nij = ni + j;
                    if self.costs[nij] > c {
                        self.costs[nij] = c;
                    }
                }
            }
        }
        self.parents.fill(usize::MAX);
        for s in 0..self.n {
            let ns = self.n * s;
            for v in 0..self.n {
                let c = self.costs[ns + v];
                if c == T::max_value() >> 1 {
                    // not connected
                    continue;
                }
                for u in 0..self.n {
                    if c == self.costs[ns + u] + self.edges[self.n * u + v] {
                        self.parents[ns + v] = u;
                        break;
                    }
                }
            }
        }
    }

    // call after floyd_warshall
    pub fn shortest_path(&self, s: usize, t: usize) -> Option<Vec<usize>> {
        debug_assert!(s < self.n);
        debug_assert!(t < self.n);
        if self.parents[t] == usize::MAX {
            return None;
        }
        let mut path = vec![t];
        let mut v = t;
        while v != s {
            v = self.parents[v];
            path.push(v);
        }
        path.reverse();
        Some(path)
    }
}
