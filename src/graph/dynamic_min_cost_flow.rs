use itertools::Itertools;

#[derive(Clone)]
pub struct Edge<T> {
    pub to: usize,
    pub cap: T,
    pub cost: T,
    pub rev: usize,
    pub is_fwd: bool,
}

pub struct DynamicMinCostFlow<T> {
    n: usize,
    edges: Vec<Vec<Edge<T>>>,
    unused_vertices: Vec<usize>,
    dual: Vec<T>,
    supplies: Vec<T>,
    cost: T,
    dist: Vec<T>,
    prev_e: Vec<usize>,
    vis: Vec<bool>,
    que_min: Vec<usize>,
    que: std::collections::BinaryHeap<std::cmp::Reverse<(T, usize)>>,
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
            dist: vec![T::zero(); n],
            prev_e: vec![0; n],
            vis: vec![false; n],
            que_min: Vec::new(),
            que: std::collections::BinaryHeap::new(),
        }
    }

    pub fn get_cost(&self) -> T {
        self.cost
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

        self.dist.fill(T::zero());
        self.vis.fill(false);
        self.que_min.clear();
        self.que.clear();

        self.dist[s] = cost;
        self.que_min.push(s);

        while !self.que_min.is_empty() || !self.que.is_empty() {
            let v = if !self.que_min.is_empty() {
                self.que_min.pop().unwrap()
            } else {
                self.que.pop().unwrap().0.1
            };
            if self.vis[v] {
                continue;
            }
            self.vis[v] = true;
            let cost_v = self.dist[v];
            self.dual[v] += cost_v;
            let dual_v = self.dual[v];
            for e in &self.edges[v] {
                if e.cap == T::zero() {
                    continue;
                }
                let cost_to = e.cost - self.dual[e.to] + dual_v;
                if cost_to < self.dist[e.to] {
                    self.dist[e.to] = cost_to;
                    if cost == cost_v {
                        self.que_min.push(e.to);
                    } else {
                        self.que.push(std::cmp::Reverse((cost_to, e.to)));
                    }
                }
            }
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: T, cost: T) -> usize {
        debug_assert!(from < self.n);
        debug_assert!(to < self.n);
        debug_assert_ne!(from, to);
        debug_assert!(cap >= T::zero());
        debug_assert!(cost >= T::zero());
        let reduced_cost = cost + self.dual[from] - self.dual[to];
        if reduced_cost >= T::zero() {
            return self.internal_add_edge(from, to, cap, cost);
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
        self.cost += flow * cost;
        fwd
    }

    pub fn remove_edge(&mut self, from: usize, fwd: usize) {
        let e_fwd = self.edges[from][fwd].clone();
        debug_assert!(e_fwd.is_fwd);
        let to = e_fwd.to;
        let rev = e_fwd.rev;
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
        self.cost -= e_fwd.cost * flow;
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
            self.dist.push(T::zero());
            self.prev_e.push(0);
            self.vis.push(false);
            ret
        }
    }

    pub fn remove_vertex(&mut self, u: usize) {
        while let Some(e) = self.edges[u].last() {
            if e.is_fwd {
                self.remove_edge(u, self.edges[u].len() - 1);
            } else {
                self.remove_edge(e.to, e.rev);
            }
        }
        self.unused_vertices.push(u);
    }

    pub fn flow(&mut self) -> T {
        self.flow_with_limits(T::max_value(), T::max_value())
    }

    // return the amount of the flow
    pub fn flow_with_limits(&mut self, flow_limit: T, cost_limit: T) -> T {
        let sources = (0..self.n).filter(|&v| self.supplies[v] > T::zero()).collect_vec();
        if sources.is_empty() {
            return T::zero();
        }
        let min_potential = sources.iter().map(|&v| self.dual[v]).min().unwrap();
        for p in &mut self.dual {
            *p -= min_potential;
        }
        let mut flow = T::zero();

        while flow < flow_limit {
            self.dist.fill(T::max_value());
            self.vis.fill(false);
            self.que_min.clear();
            self.que.clear();

            for &s in &sources {
                if self.supplies[s] > T::zero() {
                    // source
                    let cost = -self.dual[s];
                    self.dist[s] = cost;
                    self.que.push(std::cmp::Reverse((cost, s)));
                }
            }
            let mut t = usize::MAX;
            while !self.que_min.is_empty() || !self.que.is_empty() {
                let v = if !self.que_min.is_empty() {
                    self.que_min.pop().unwrap()
                } else {
                    self.que.pop().unwrap().0.1
                };
                if self.vis[v] {
                    continue;
                }
                self.vis[v] = true;
                if self.supplies[v] < T::zero() {
                    t = v;
                    break;
                }
                let dual_v = self.dual[v];
                let dist_v = self.dist[v];
                for e in &self.edges[v] {
                    if e.cap == T::zero() {
                        continue;
                    }
                    let cost = e.cost - self.dual[e.to] + dual_v;
                    debug_assert!(cost >= T::zero());
                    if self.dist[e.to] - dist_v > cost {
                        let dist_to = dist_v + cost;
                        if dist_to > cost_limit {
                            continue;
                        }
                        self.dist[e.to] = dist_to;
                        self.prev_e[e.to] = e.rev;
                        if dist_to == dist_v {
                            self.que_min.push(e.to);
                        } else {
                            self.que.push(std::cmp::Reverse((dist_to, e.to)));
                        }
                    }
                }
            }
            if t == usize::MAX {
                break;
            }
            for v in 0..self.n {
                if !self.vis[v] {
                    continue;
                }
                self.dual[v] -= self.dist[t] - self.dist[v];
            }

            let mut c = (flow_limit - flow).min(-self.supplies[t]);
            let mut v = t;
            while self.supplies[v] <= T::zero() {
                let e = &self.edges[v][self.prev_e[v]];
                c = c.min(self.edges[e.to][e.rev].cap);
                v = e.to;
            }
            let s = v;
            c = c.min(self.supplies[s]);
            v = t;
            while self.supplies[v] <= T::zero() {
                self.edges[v][self.prev_e[v]].cap += c;
                let fwd = self.edges[v][self.prev_e[v]].rev;
                v = self.edges[v][self.prev_e[v]].to;
                self.edges[v][fwd].cap -= c;
            }
            let d = self.dual[t] - self.dual[s];
            flow += c;
            self.cost += c * d;
        }
        flow
    }
}