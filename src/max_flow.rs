pub struct Edge<Cap> {
    from: usize,
    to: usize,
    cap: Cap,
    flow: Cap,
}

#[derive(Clone)]
struct _Edge<Cap> {
    to: usize,
    rev: usize,
    cap: Cap,
}

struct MaxFlow<Cap> {
    n: usize,
    pos: Vec<(usize, usize)>,
    g: Vec<Vec<_Edge<Cap>>>,
    level: Vec<i32>,
    iter: Vec<usize>,
    que: std::collections::VecDeque<usize>,
}

impl<Cap: num_traits::NumAssign + num_traits::PrimInt> MaxFlow<Cap> {
    pub fn new(n: usize) -> Self {
        MaxFlow {
            n,
            pos: Vec::new(),
            g: vec![vec![]; n],
            level: Vec::new(),
            iter: Vec::new(),
            que: std::collections::VecDeque::new(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: Cap) -> usize {
        debug_assert!(from < self.n);
        debug_assert!(to < self.n);
        debug_assert!(cap >= Cap::zero());
        let m = self.pos.len();
        self.pos.push((from, self.g[from].len()));
        let from_id = self.g[from].len();
        let mut to_id = self.g[to].len();
        if from == to {
            to_id += 1;
        }
        self.g[from].push(_Edge {
            to,
            rev: to_id,
            cap,
        });
        self.g[to].push(_Edge {
            to: from,
            rev: from_id,
            cap: Cap::zero(),
        });
        m
    }

    pub fn get_edge(&self, i: usize) -> Edge<Cap> {
        debug_assert!(i < self.pos.len());
        let e = &self.g[self.pos[i].0][self.pos[i].1];
        let re = &self.g[e.to][e.rev];
        Edge {
            from: self.pos[i].0,
            to: e.to,
            cap: e.cap + re.cap,
            flow: re.cap,
        }
    }

    pub fn edges(&self) -> Vec<Edge<Cap>> {
        let m = self.pos.len();
        let mut result = Vec::with_capacity(m);
        for i in 0..m {
            result.push(self.get_edge(i));
        }
        result
    }

    pub fn change_edge(&mut self, i: usize, new_cap: Cap, new_flow: Cap) {
        debug_assert!(i < self.pos.len());
        debug_assert!(new_flow <= new_cap);
        self.g[self.pos[i].0][self.pos[i].1].cap = new_cap - new_flow;
        let e_to = self.g[self.pos[i].0][self.pos[i].1].to;
        let e_rev = self.g[self.pos[i].0][self.pos[i].1].rev;
        self.g[e_to][e_rev].cap = new_flow;
    }

    pub fn flow(&mut self, s: usize, t: usize) -> Cap {
        self.flow_with_limit(s, t, Cap::max_value())
    }

    pub fn flow_with_limit(&mut self, s: usize, t: usize, flow_limit: Cap) -> Cap {
        debug_assert!(s < self.n);
        debug_assert!(t < self.n);
        debug_assert_ne!(s, t);

        self.level.resize(self.n, -1);
        self.iter.resize(self.n, 0);
        self.que.reserve(self.n);

        let mut flow = Cap::zero();
        while flow < flow_limit {
            self.bfs(s, t);
            if self.level[t] == -1 {
                break;
            }
            self.iter.fill(0);
            let f = self.dfs(t, flow_limit - flow, s);
            if f == Cap::zero() {
                break;
            }
            flow += f;
        }
        flow
    }

    fn bfs(&mut self, s: usize, t: usize) {
        self.level.fill(-1);
        self.level[s] = 0;
        self.que.clear();
        self.que.push_back(s);
        while !self.que.is_empty() {
            let v = self.que.pop_front().unwrap();
            for e in self.g[v].iter() {
                if e.cap == Cap::zero() || self.level[e.to] >= 0 {
                    continue;
                }
                self.level[e.to] = self.level[v] + 1;
                if e.to == t {
                    return;
                }
                self.que.push_back(e.to);
            }
        }
    }

    fn dfs(&mut self, v: usize, up: Cap, s: usize) -> Cap {
        if v == s {
            return up;
        }
        let mut res = Cap::zero();
        let level_v = self.level[v];
        for i in self.iter[v]..self.g[v].len() {
            self.iter[v] = i;
            let e = self.g[v][i].clone();
            if level_v <= self.level[e.to] || self.g[e.to][e.rev].cap == Cap::zero() {
                continue;
            }
            let d = self.dfs(e.to, (up - res).min(self.g[e.to][e.rev].cap), s);
            if d <= Cap::zero() {
                continue;
            }
            self.g[v][i].cap = d;
            self.g[e.to][e.rev].cap -= d;
            res += d;
            if res == up {
                return res;
            }
        }
        self.level[v] = self.n as i32;
        res
    }

    pub fn min_cut(&mut self, s: usize) -> Vec<bool> {
        let mut visited = vec![false; self.n];
        self.que.clear();
        self.que.push_back(s);
        while !self.que.is_empty() {
            let p = self.que.pop_front().unwrap();
            visited[p] = true;
            for e in self.g[p].iter() {
                if e.cap > Cap::zero() && !visited[e.to] {
                    visited[e.to] = true;
                    self.que.push_back(e.to);
                }
            }
        }
        visited
    }
}
