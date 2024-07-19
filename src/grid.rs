pub struct Grid {
    h: usize,
    w: usize,
    n: usize,
    edges: Vec<Vec<usize>>,
    costs: Vec<usize>,
    todo: std::collections::VecDeque<usize>,
}

impl Grid {
    pub fn no_walls(h: usize, w: usize) -> Self {
        let n = h * w;
        let mut edges = vec![Vec::with_capacity(4); n];
        for x in 0..h {
            for y in 0..w {
                let v = w * x + y;
                if x > 0 {
                    edges[v].push(v - w);
                }
                if y > 0 {
                    edges[v].push(v - 1);
                }
                if y + 1 < w {
                    edges[v].push(v + 1);
                }
                if x + 1 < h {
                    edges[v].push(v + w);
                }
            }
        }
        Grid {
            h,
            w,
            n,
            edges,
            costs: vec![0; n],
            todo: std::collections::VecDeque::with_capacity(n),
        }
    }

    // vertical: &[[bool; h]; w - 1]
    // horizontal: &[[bool; w]; h - 1]
    pub fn from_walls(vertical: &[&[bool]], horizontal: &[&[bool]]) -> Self {
        debug_assert!(!horizontal.is_empty());
        let h = vertical.len();
        let w = horizontal[0].len();
        debug_assert!(h > 0 && w > 0);
        debug_assert_eq!(vertical[0].len(), w - 1);
        debug_assert_eq!(horizontal.len(), h - 1);

        let n = h * w;
        let mut edges = vec![Vec::with_capacity(4); n];
        for x in 0..h {
            for y in 0..w {
                let v = w * x + y;
                if x > 0 && !horizontal[x - 1][y] {
                    edges[v].push(v - w);
                }
                if y > 0 && !vertical[x][y - 1] {
                    edges[v].push(v - 1);
                }
                if y + 1 < w && !vertical[x][y] {
                    edges[v].push(v + 1);
                }
                if x + 1 < h && !horizontal[x][y] {
                    edges[v].push(v + w);
                }
            }
        }
        Grid {
            h,
            w,
            n,
            edges,
            costs: vec![0; n],
            todo: std::collections::VecDeque::with_capacity(n),
        }
    }

    // blocked: &[[bool; h]; w]
    pub fn from_blocked(blocked: &[&[bool]]) -> Self {
        debug_assert!(!blocked.is_empty());
        let h = blocked.len();
        let w = blocked[0].len();
        let n = h * w;
        let mut edges = vec![Vec::with_capacity(4); n];
        for x in 0..h {
            for y in 0..w {
                if blocked[x][y] {
                    continue;
                }
                let v = w * x + y;
                if x > 0 && !blocked[x - 1][y] {
                    edges[v].push(v - w);
                }
                if y > 0 && !blocked[x][y - 1] {
                    edges[v].push(v - 1);
                }
                if y + 1 < w && !blocked[x][y + 1] {
                    edges[v].push(v + 1);
                }
                if x + 1 < h && !blocked[x + 1][y] {
                    edges[v].push(v + w);
                }
            }
        }
        Grid {
            h,
            w,
            n,
            edges,
            costs: vec![0; n],
            todo: std::collections::VecDeque::with_capacity(n),
        }
    }

    pub fn encode(&self, x: usize, y: usize) -> usize {
        debug_assert!(x < self.h);
        debug_assert!(y < self.w);
        self.w * x + y
    }

    pub fn decode(&self, v: usize) -> (usize, usize) {
        debug_assert!(v < self.n);
        (v / self.w, v % self.w)
    }

    pub fn manhattan(&self, u: usize, v: usize) -> usize {
        debug_assert!(u < self.n);
        debug_assert!(v < self.n);
        (u / self.w).abs_diff(v / self.w) + (u % self.w).abs_diff(v % self.w)
    }

    pub fn euclid2(&self, u: usize, v: usize) -> usize {
        debug_assert!(u < self.n);
        debug_assert!(v < self.n);
        let dx = (u / self.w).abs_diff(v / self.w);
        let dy = (u % self.w).abs_diff(v % self.w);
        dx * dx + dy * dy
    }

    // return distance from s
    // If t == self.n, it searches all vertices.
    pub fn bfs(&mut self, s: usize, t: usize) {
        debug_assert!(s < self.n);
        debug_assert!(t <= self.n);
        debug_assert_eq!(self.costs.len(), self.n);
        self.costs.fill(usize::MAX);
        self.costs[s] = 0;
        self.todo.clear();
        self.todo.push_back(s);
        while let Some(u) = self.todo.pop_front() {
            if u == t {
                break;
            }
            for &v in &self.edges[u] {
                if self.costs[v] > self.costs[u] + 1 {
                    self.costs[v] = self.costs[u] + 1;
                    self.todo.push_back(v);
                }
            }
        }
    }

    // call after bfs
    pub fn shortest_path(&self, s: usize, t: usize) -> Option<Vec<usize>> {
        debug_assert!(s < self.n);
        debug_assert!(t < self.n);
        if self.costs[t] == usize::MAX {
            return None;
        }
        let mut path = vec![t];
        let mut v = t;
        while v != s {
            v = *self.edges[v].iter().find(|&&u| self.costs[u] + 1 == self.costs[v]).unwrap();
            path.push(v);
        }
        path.reverse();
        Some(path)
    }
}
