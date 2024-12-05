#[derive(Clone)]
pub struct Edge<T> {
    pub to: usize,
    pub cap: T,
    pub cost: T,
    pub rev: usize,
    pub is_fwd: bool,
}

#[derive(Clone)]
pub struct DynamicMinCostFlow<T> {
    n: usize,
    edges: Vec<Vec<Edge<T>>>,
    unused_vertices: Vec<usize>,
    dual: Vec<T>,
    supplies: Vec<T>,
    cost: T,
}

impl<T> DynamicMinCostFlow<T>
where T: std::fmt::Debug + num_traits::NumAssign + num_traits::PrimInt + std::ops::Neg<Output = T>,
{
    pub fn new(n: usize) -> Self {
        DynamicMinCostFlow {
            n,
            edges: vec![Vec::new(); n],
            unused_vertices: Vec::new(),
            dual: vec![T::zero(); n],
            supplies: vec![T::zero(); n],
            cost: T::zero(),
        }
    }

    pub fn get_edges(&self) -> &Vec<Vec<Edge<T>>> {
        &self.edges
    }

    pub fn get_cost(&self) -> T {
        self.cost
    }

    pub fn add_supply(&mut self, v: usize, supply: T) {
        debug_assert!(v < self.n);
        self.supplies[v] += supply;
    }

    fn internal_add_edge(&mut self, from: usize, to: usize, cap: T, cost: T) -> usize {
        let fwd = self.edges[from].len();
        let rev = self.edges[to].len();
        self.edges[from].push(Edge {
            to,
            cap,
            cost,
            rev,
            is_fwd: true,
        });
        self.edges[to].push(Edge {
            to: from,
            cap: T::zero(),
            cost: -cost,
            rev: fwd,
            is_fwd: false,
        });
        fwd
    }

    // update dual
    fn change_to_non_negative_cost(&mut self, s: usize, cost: T) {
        debug_assert!(cost < T::zero());

        let mut costs = vec![T::zero(); self.n];
        let mut que_min = Vec::new();
        let mut que: std::collections::BinaryHeap<std::cmp::Reverse<(T, usize)>> = std::collections::BinaryHeap::new();

        costs[s] = cost;
        que_min.push((cost, s));

        while !que_min.is_empty() || !que.is_empty() {
            let (cost_v, v) = if !que_min.is_empty() {
                que_min.pop().unwrap()
            } else {
                que.pop().unwrap().0
            };
            if costs[v] < cost_v {
                continue;
            }
            self.dual[v] += cost_v;
            let dual_v = self.dual[v];
            for e in &self.edges[v] {
                let e = e.clone();
                if e.cap == T::zero() {
                    continue;
                }
                let cost_to = e.cost - self.dual[e.to] + dual_v;
                debug_assert!(cost_to >= cost_v);
                if cost_to < costs[e.to] {
                    costs[e.to] = cost_to;
                    if cost_to == cost_v {
                        que_min.push((cost_to, e.to));
                    } else {
                        que.push(std::cmp::Reverse((cost_to, e.to)));
                    }
                }
            }
        }
    }

    // For speed-up, add edges to new vertices before adding edges from new vertices.
    pub fn add_edge(&mut self, from: usize, to: usize, cap: T, cost: T) {
        debug_assert!(from < self.n);
        debug_assert!(to < self.n);
        debug_assert_ne!(from, to);
        debug_assert!(cap >= T::zero());
        debug_assert!(cost >= T::zero());

        let reduced_cost = cost + self.dual[from] - self.dual[to];
        if reduced_cost >= T::zero() {
            // positive cost
            self.internal_add_edge(from, to, cap, cost);
            return;
        }
        if self.edges[to].iter().all(|e| e.cap == T::zero() || e.cost + self.dual[to] - self.dual[e.to] + reduced_cost >= T::zero()) {
            // can update potential
            self.dual[to] += reduced_cost;
            self.internal_add_edge(from, to, cap, cost);
            return;
        }
        // flow along the minimum cost negative cycle
        let mut original_supplies = vec![T::zero(); self.n];
        std::mem::swap(&mut original_supplies, &mut self.supplies);
        self.supplies[to] += cap;
        self.supplies[from] -= cap;
        let flow = self.flow_with_limits(cap, -reduced_cost);
        debug_assert_eq!(self.supplies[to], cap - flow);
        debug_assert_eq!(self.supplies[from], flow - cap);
        std::mem::swap(&mut original_supplies, &mut self.supplies);

        let reduced_cost = cost + self.dual[from] - self.dual[to];
        if reduced_cost < T::zero() {
            self.change_to_non_negative_cost(to, reduced_cost);
        }
        // flow along the adding edge
        let fwd = self.internal_add_edge(from, to, cap, cost);
        let rev = self.edges[from][fwd].rev;
        self.edges[from][fwd].cap -= flow;
        self.edges[to][rev].cap += flow;
        self.cost += cost * flow;
    }

    fn internal_remove_edge(&mut self, from: usize, fwd: usize) {
        let e_fwd = self.edges[from][fwd].clone();
        debug_assert!(e_fwd.is_fwd);
        let to = e_fwd.to;
        let rev = e_fwd.rev;
        let cost = e_fwd.cost;
        let flow = self.edges[to][rev].cap;

        self.edges[from].swap_remove(fwd);
        if fwd < self.edges[from].len() {
            let e = self.edges[from][fwd].clone();
            self.edges[e.to][e.rev].rev = fwd;
        }
        self.edges[to].swap_remove(rev);
        if rev < self.edges[to].len() {
            let e = self.edges[to][rev].clone();
            self.edges[e.to][e.rev].rev = rev;
        }
        self.supplies[from] += flow;
        self.supplies[to] -= flow;
        self.cost -= cost * flow;
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) {
        for i in (0..self.edges[from].len()).rev() {
            let e = self.edges[from][i].clone();
            if e.to == to && e.is_fwd {
                self.internal_remove_edge(from, i);
            }
        }
    }

    // return an isolated vertex
    pub fn add_vertex(&mut self) -> usize {
        if let Some(ret) = self.unused_vertices.pop() {
            ret
        } else {
            let ret = self.n;
            self.n += 1;
            self.edges.push(Vec::new());
            self.dual.push(T::zero());
            self.supplies.push(T::zero());
            ret
        }
    }

    pub fn remove_vertex(&mut self, v: usize) {
        while let Some(e) = self.edges[v].last() {
            if e.is_fwd {
                self.internal_remove_edge(v, self.edges[v].len() - 1);
            } else {
                self.internal_remove_edge(e.to, e.rev);
            }
        }
        self.unused_vertices.push(v);
        self.dual[v] = T::zero();
        self.supplies[v] = T::zero();
    }

    pub fn flow(&mut self) -> T {
        self.flow_with_limits(T::max_value(), T::max_value())
    }

    // return the amount of the flow
    fn flow_with_limits(&mut self, flow_limit: T, cost_limit: T) -> T {
        let sources: Vec<usize> = (0..self.n).filter(|&v| self.supplies[v] > T::zero()).collect();
        if sources.is_empty() {
            return T::zero();
        }
        let min_potential = sources.iter().map(|&v| self.dual[v]).min().unwrap();
        for p in &mut self.dual {
            *p -= min_potential;
        }
        let mut dist = vec![T::zero(); self.n];
        let mut prev_e = vec![0; self.n];
        let mut vis = vec![false; self.n];

        let mut que_min = Vec::new();
        let mut que = std::collections::BinaryHeap::new();

        let mut flow = T::zero();

        while flow < flow_limit {
            dist.fill(T::max_value() >> 1);
            vis.fill(false);
            que_min.clear();
            que.clear();

            for &s in &sources {
                if self.supplies[s] > T::zero() {
                    let cost = -self.dual[s];
                    dist[s] = cost;
                    prev_e[s] = usize::MAX;
                    que.push(std::cmp::Reverse((cost, s)));
                }
            }
            let mut t = usize::MAX;
            while !que_min.is_empty() || !que.is_empty() {
                let v = if !que_min.is_empty() {
                    que_min.pop().unwrap()
                } else {
                    que.pop().unwrap().0.1
                };
                if vis[v] {
                    continue;
                }
                vis[v] = true;
                if self.supplies[v] < T::zero() {
                    t = v;
                    break;
                }
                let dual_v = self.dual[v];
                let dist_v = dist[v];
                for e in &self.edges[v] {
                    let e = e.clone();
                    if e.cap == T::zero() {
                        continue;
                    }
                    let cost = e.cost - self.dual[e.to] + dual_v;
                    debug_assert!(cost >= T::zero());
                    if dist[e.to] - dist_v > cost {
                        let dist_to = dist_v + cost;
                        if dist_to > cost_limit {
                            continue;
                        }
                        dist[e.to] = dist_to;
                        prev_e[e.to] = e.rev;
                        if dist_to == dist_v {
                            que_min.push(e.to);
                        } else {
                            que.push(std::cmp::Reverse((dist_to, e.to)));
                        }
                    }
                }
            }
            if t == usize::MAX {
                break;
            }
            for v in 0..self.n {
                if !vis[v] {
                    continue;
                }
                self.dual[v] -= dist[t] - dist[v];
            }
            // restore path
            let mut f = (flow_limit - flow).min(-self.supplies[t]);
            let mut v = t;
            while prev_e[v] != usize::MAX {
                let e = &self.edges[v][prev_e[v]];
                f = f.min(self.edges[e.to][e.rev].cap);
                v = e.to;
            }
            let s = v;
            debug_assert!(self.supplies[s] > T::zero());
            f = f.min(self.supplies[s]);

            // update capacity
            let mut v = t;
            while v != s {
                self.edges[v][prev_e[v]].cap += f;
                let fwd = self.edges[v][prev_e[v]].rev;
                v = self.edges[v][prev_e[v]].to;
                self.edges[v][fwd].cap -= f;
            }
            let d = self.dual[t] - self.dual[s];
            flow += f;
            self.cost += f * d;
            self.supplies[s] -= f;
            self.supplies[t] += f;
        }
        flow
    }
}
