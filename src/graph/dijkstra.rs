pub struct Dijkstra<T> {
    n: usize,
    edges: Vec<Vec<(usize, T)>>,
    costs: Vec<T>,
    parents: Vec<usize>,
}

impl<T: num_traits::NumAssign + num_traits::PrimInt> Dijkstra<T> {
    pub fn new(n: usize) -> Self {
        Dijkstra {
            n,
            edges: vec![Vec::new(); n],
            costs: vec![T::zero(); n],
            parents: vec![0; n],
        }
    }

    pub fn add_directed_edge(&mut self, u: usize, v: usize, c: T) {
        debug_assert!(u < self.n);
        debug_assert!(v < self.n);
        self.edges[u].push((v, c));
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize, c: T) {
        debug_assert!(u < self.n);
        debug_assert!(v < self.n);
        self.add_directed_edge(u, v, c);
        self.add_directed_edge(v, u, c);
    }

    // return distance from s
    // If t == self.n, it searches all vertices.
    pub fn dijkstra(&mut self, s: usize, t: usize) {
        debug_assert!(s < self.n);
        debug_assert!(t <= self.n);
        debug_assert_eq!(self.costs.len(), self.n);
        debug_assert_eq!(self.parents.len(), self.n);
        self.costs.fill(T::max_value());
        self.costs[s] = T::zero();
        self.parents.fill(usize::MAX);
        self.parents[s] = s;
        let mut todo = std::collections::BinaryHeap::with_capacity(self.n);
        todo.push(std::cmp::Reverse((T::zero(), s)));
        while let Some(std::cmp::Reverse((cost, u))) = todo.pop() {
            if self.costs[u] < cost {
                continue;
            }
            if u == t {
                break;
            }
            for &(v, mut c) in &self.edges[u] {
                c += cost;
                if c < self.costs[v] {
                    self.costs[v] = c;
                    self.parents[v] = u;
                    todo.push(std::cmp::Reverse((c, v)));
                }
            }
        }
    }

    // call after dijkstra
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
