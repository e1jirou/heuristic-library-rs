#[derive(Debug, Clone)]
pub struct Edge<T> {
    pub to: usize,
    pub cap: T,
    pub cost: T,
    pub rev: usize,
    pub is_fwd: bool,
}

#[derive(Debug, Clone)]
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

    pub fn set_supply(&mut self, v: usize, supply: T) {
        debug_assert!(v < self.n);
        self.supplies[v] = supply;
    }

    pub fn get_supply(&self, v: usize) -> T {
        debug_assert!(v < self.n);
        self.supplies[v]
    }

    pub fn add_supply(&mut self, v: usize, additional: T) {
        debug_assert!(v < self.n);
        self.supplies[v] += additional;
    }

    fn add_edge_internal(&mut self, from: usize, to: usize, cap: T, cost: T) -> usize {
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

    // decrease potential[s] to remove negative cost edge
    fn decrease_potential(&mut self, s: usize, cost: T) {
        debug_assert!(cost < T::zero());

        let mut costs = vec![T::zero(); self.n];
        let mut que_min = Vec::new();
        let mut que: std::collections::BinaryHeap<std::cmp::Reverse<(T, usize)>> = std::collections::BinaryHeap::new();

        costs[s] = cost;
        que_min.push((cost, s));

        // Dijkstra
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

    // flow along the minimum cost negative cycle
    fn remove_negative_cycles(&mut self, from: usize, to: usize, cap: T, reduced_cost: T) -> T {
        debug_assert!(reduced_cost < T::zero());

        // evacuation
        let mut original_supplies = vec![T::zero(); self.n];
        std::mem::swap(&mut original_supplies, &mut self.supplies);

        // update except an adding edge
        self.supplies[to] += cap;
        self.supplies[from] -= cap;
        let flow = self.flow_with_limits(cap, -reduced_cost);
        debug_assert_eq!(self.supplies[to], cap - flow);
        debug_assert_eq!(self.supplies[from], flow - cap);

        // restore
        std::mem::swap(&mut original_supplies, &mut self.supplies);

        flow
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: T, cost: T) {
        debug_assert!(from < self.n);
        debug_assert!(to < self.n);
        debug_assert_ne!(from, to);
        debug_assert!(cap >= T::zero());
        debug_assert!(cost >= T::zero());

        let reduced_cost = cost + self.dual[from] - self.dual[to];
        if reduced_cost >= T::zero() {
            // positive cost
            self.add_edge_internal(from, to, cap, cost);
            return;
        }
        if self.edges[to].iter().all(|e| e.cap == T::zero() || e.cost + (self.dual[to] + reduced_cost) - self.dual[e.to] >= T::zero()) {
            // not generate nagative cost by decrease of self.dual[to]
            self.dual[to] += reduced_cost;
            self.add_edge_internal(from, to, cap, cost);
            return;
        }
        if self.edges[from].iter().all(|e| self.edges[e.to][e.rev].cap == T::zero() || self.edges[e.to][e.rev].cost + self.dual[e.to] - (self.dual[from] - reduced_cost) >= T::zero()) {
            // not generate negative cost by increase of self.dual[from]
            self.dual[from] -= reduced_cost;
            self.add_edge_internal(from, to, cap, cost);
            return;
        }

        let flow = self.remove_negative_cycles(from, to, cap, reduced_cost);

        let reduced_cost = cost + self.dual[from] - self.dual[to];
        if reduced_cost < T::zero() {
            self.decrease_potential(to, reduced_cost);
        }
        // flow along the adding edge
        let fwd = self.add_edge_internal(from, to, cap, cost);
        let rev = self.edges[from][fwd].rev;
        self.edges[from][fwd].cap -= flow;
        self.edges[to][rev].cap += flow;
        self.cost += cost * flow;
    }

    fn remove_edge_internal(&mut self, from: usize, fwd: usize) {
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

    // remove all edges (from, to)
    pub fn remove_edge(&mut self, from: usize, to: usize) {
        debug_assert!(from < self.n);
        debug_assert!(to < self.n);
        for i in (0..self.edges[from].len()).rev() {
            let e = self.edges[from][i].clone();
            if e.to == to && e.is_fwd {
                self.remove_edge_internal(from, i);
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
        debug_assert!(v < self.n);
        // remove edges
        while let Some(e) = self.edges[v].last() {
            if e.is_fwd {
                self.remove_edge_internal(v, self.edges[v].len() - 1);
            } else {
                self.remove_edge_internal(e.to, e.rev);
            }
        }
        // remove the vertex
        self.unused_vertices.push(v);
        self.dual[v] = T::zero();
        self.supplies[v] = T::zero();
    }

    // not support multiple edges
    pub fn increase_cap(&mut self, from: usize, to: usize, increase: T) {
        debug_assert!(from < self.n);
        debug_assert!(to < self.n);
        debug_assert!(increase >= T::zero());
        if increase == T::zero() {
            return;
        }
        let i = self.edges[from].iter().position(|e| e.to == to && e.is_fwd).unwrap();
        let increase_edge = self.edges[from][i].clone();

        if increase_edge.cap > T::zero() {
            // excess capacity
            self.edges[from][i].cap += increase;
            return;
        }

        // similar to edge addition in the residual network
        let reduced_cost = increase_edge.cost + self.dual[from] - self.dual[to];
        if reduced_cost >= T::zero() {
            self.edges[from][i].cap += increase;
            return;
        }
        if self.edges[to].iter().all(|e| e.cap == T::zero() || e.cost + (self.dual[to] + reduced_cost) - self.dual[e.to] >= T::zero()) {
            // not generate negative cost by decrease of self.dual[to]
            self.dual[to] += reduced_cost;
            self.edges[from][i].cap += increase;
            return;
        }
        if self.edges[from].iter().all(|e| self.edges[e.to][e.rev].cap == T::zero() || self.edges[e.to][e.rev].cost + self.dual[e.to] - (self.dual[from] - reduced_cost) >= T::zero()) {
            // not generate negative cost by increase of self.dual[from]
            self.dual[from] -= reduced_cost;
            self.edges[from][i].cap += increase;
            return;
        }

        let flow = self.remove_negative_cycles(from, to, increase, reduced_cost);

        let reduced_cost = increase_edge.cost + self.dual[from] - self.dual[to];

        if reduced_cost < T::zero() {
            self.decrease_potential(to, reduced_cost);
        }

        // flow along the adding edge
        self.edges[from][i].cap += increase - flow;
        self.edges[to][increase_edge.rev].cap += flow;
        self.cost += increase_edge.cost * flow;
    }

    // not support multiple edges
    pub fn decrease_cap(&mut self, from: usize, to: usize, decrease: T) {
        debug_assert!(from < self.n);
        debug_assert!(to < self.n);
        debug_assert!(decrease >= T::zero());
        if decrease == T::zero() {
            return;
        }
        let i = self.edges[from].iter().position(|e| e.to == to && e.is_fwd).unwrap();
        let decrease_edge = self.edges[from][i].clone();
        debug_assert!(decrease_edge.cap + self.edges[to][decrease_edge.rev].cap >= decrease);

        if decrease_edge.cap >= decrease {
            // excess capacity
            self.edges[from][i].cap -= decrease;
            return;
        }

        let decrease = decrease_edge.cap;
        self.edges[from][i].cap = T::zero();

        // decrease reverse edge capacity
        self.edges[to][decrease_edge.rev].cap -= decrease;
        self.supplies[from] += decrease;
        self.supplies[to] -= decrease;
        self.cost -= decrease_edge.cost * decrease;
    }

    pub fn flow(&mut self) -> T {
        self.flow_with_limits(T::max_value(), T::max_value())
    }

    // flow from supplies to s, from t to demands
    pub fn reverse_flow(&mut self, s: usize, t: usize) {
        debug_assert!(s < self.n);
        debug_assert!(t < self.n);
        let s_supply = self.supplies[s];
        let t_supply = self.supplies[t];
        self.supplies[s] = T::zero();
        self.supplies[t] = T::zero();

        let mut sum_supplies = T::zero();
        let mut sum_demands = T::zero();
        for &supply in &self.supplies {
            if supply >= T::zero() {
                sum_supplies += supply;
            } else {
                sum_demands -= supply;
            }
        }

        if sum_supplies > T::zero() {
            self.supplies[s] = -sum_supplies;
            self.flow();
            self.supplies[s] += sum_supplies;
        }

        if sum_demands > T::zero() {
            self.supplies[t] = sum_demands;
            self.flow();
            self.supplies[t] -= sum_demands;
        }

        self.supplies[s] += s_supply;
        self.supplies[t] += t_supply;
    }

    // return the amount of the flow
    fn flow_with_limits(&mut self, flow_limit: T, dist_limit: T) -> T {
        let sources: Vec<usize> = (0..self.n).filter(|&v| self.supplies[v] > T::zero()).collect();
        if sources.is_empty() {
            return T::zero();
        }
        // normalize potential
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
            // Dijkstra

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
                    let dist_to = dist_v + cost;
                    if dist_to >= dist[e.to] || dist_to > dist_limit {
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
            if t == usize::MAX {
                break;
            }
            for v in 0..self.n {
                if vis[v] {
                    self.dual[v] -= dist[t] - dist[v];
                }
            }
            // restore path
            let mut f = (flow_limit - flow).min(-self.supplies[t]); // flow amount
            let mut v = t;
            while prev_e[v] != usize::MAX {
                let e = &self.edges[v][prev_e[v]]; // reverse edge
                f = f.min(self.edges[e.to][e.rev].cap);
                v = e.to;
            }
            let s = v; // supply
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
            let d = self.dual[t] - self.dual[s]; // cost per amount
            flow += f;
            self.cost += f * d;
            self.supplies[s] -= f;
            self.supplies[t] += f;
        }
        flow
    }
}
