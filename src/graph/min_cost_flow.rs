fn csr<T: Clone + Default>(n: usize, edges: &Vec<(usize, T)>) -> (Vec<usize>, Vec<T>) {
    let mut start = vec![0; n + 1];
    for (u, _) in edges.iter() {
        start[u + 1] += 1;
    }
    for i in 0..n {
        start[i + 1] += start[i];
    }
    let mut counter = start.clone();
    let mut elist = vec![T::default(); edges.len()];
    for (u, v) in edges.iter() {
        elist[counter[*u]] = v.clone();
        counter[*u] += 1;
    }
    (start, elist)
}

#[derive(Clone)]
pub struct Edge<T> {
    pub from: usize,
    pub to: usize,
    pub cap: T,
    pub flow: T,
    pub cost: T,
}

#[derive(Clone, Default)]
struct _Edge<T> {
    to: usize,
    rev: usize,
    cap: T,
    cost: T,
}

pub struct MinCostFlow<T> {
    n: usize,
    edges: Vec<Edge<T>>,
}

impl<T> MinCostFlow<T>
where
    T: Default + num_traits::NumAssign + num_traits::PrimInt + std::ops::Neg<Output = T>,
{
    pub fn new(n: usize) -> Self {
        MinCostFlow {
            n,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: T, cost: T) -> usize {
        debug_assert!(from < self.n);
        debug_assert!(to < self.n);
        debug_assert!(cap >= T::zero());
        debug_assert!(cost >= T::zero());
        let m = self.edges.len();
        self.edges.push(Edge {
            from,
            to,
            cap,
            flow: T::zero(),
            cost,
        });
        m
    }

    pub fn get_edge(&self, i: usize) -> &Edge<T> {
        debug_assert!(i < self.edges.len());
        &self.edges[i]
    }

    pub fn edges(&self) -> &Vec<Edge<T>> {
        &self.edges
    }

    // return the amount of the flow and the cost
    pub fn flow(&mut self, s: usize, t: usize) -> (T, T) {
        self.flow_with_limit(s, t, T::max_value())
    }

    // return the amount of the flow and the cost
    pub fn flow_with_limit(&mut self, s: usize, t: usize, flow_limit: T) -> (T, T) {
        *self.slope(s, t, flow_limit).last().unwrap()
    }

    pub fn slope(&mut self, s: usize, t: usize, flow_limit: T) -> Vec<(T, T)> {
        debug_assert!(s < self.n);
        debug_assert!(t < self.n);
        debug_assert_ne!(s, t);

        let m = self.edges.len();
        let mut edge_idx = vec![0; m];

        let (start, mut elist) = self.make_csr(&mut edge_idx);

        // slope
        // variants (C = maxcost):
        // -(n-1)C <= dual[s] <= dual[i] <= dual[t] = 0
        // reduced cost (= e.cost + dual[e.from] - dual[e.to]) >= 0 for all edge

        // dual_dist[i] = (dual[i], dist[i])
        let mut dual_dist = vec![(T::zero(), T::zero()); self.n];
        let mut prev_e = vec![0; self.n];
        let mut vis = vec![false; self.n];

        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        struct Q<Cost> {
            key: Cost,
            to: usize,
        }
        let mut que_min = Vec::new();
        let mut que: std::collections::BinaryHeap<std::cmp::Reverse<Q<T>>> = std::collections::BinaryHeap::new();

        let mut flow = T::zero();
        let mut cost = T::zero();
        let mut prev_cost_per_flow = -T::one();
        let mut result = vec![(T::zero(), T::zero())];
        while flow < flow_limit {
            // dual ref
            for i in 0..self.n {
                dual_dist[i].1 = T::max_value();
            }
            vis.fill(false);
            que_min.clear();
            que.clear();

            dual_dist[s].1 = T::zero();
            que_min.push(s);
            while !que_min.is_empty() || !que.is_empty() {
                let v = if !que_min.is_empty() {
                    que_min.pop().unwrap()
                } else {
                    que.pop().unwrap().0.to
                };
                if vis[v] {
                    continue;
                }
                vis[v] = true;
                if v == t {
                    break;
                }
                // dist[v] = shortest(s, v) + dual[s] - dual[v]
                // dist[v] >= 0 (all reduced cost are positive)
                // dist[v] <= (n-1)C
                let dual_v = dual_dist[v].0;
                let dist_v = dual_dist[v].1;
                for i in start[v]..start[v + 1] {
                    let e = &elist[i];
                    if e.cap == T::zero() {
                        continue;
                    }
                    // |-dual[e.to] + dual[v]| <= (n-1)C
                    // cost <= C - -(n-1)C + 0 = nC
                    let cost = e.cost - dual_dist[e.to].0 + dual_v;
                    if dual_dist[e.to].1 - dist_v > cost {
                        let dist_to = dist_v + cost;
                        dual_dist[e.to].1 = dist_to;
                        prev_e[e.to] = e.rev;
                        if dist_to == dist_v {
                            que_min.push(e.to);
                        } else {
                            que.push(std::cmp::Reverse(Q {
                                key: dist_to,
                                to: e.to,
                            }));
                        }
                    }
                }
            }
            if !vis[t] {
                break;
            }
            for v in 0..self.n {
                if !vis[v] {
                    continue;
                }
                // dual[v] = dual[v] - dist[t] + dist[v]
                //         = dual[v] - (shortest(s, t) + dual[s] - dual[t]) +
                //         (shortest(s, v) + dual[s] - dual[v]) = - shortest(s,
                //         t) + dual[t] + shortest(s, v) = shortest(s, v) -
                //         shortest(s, t) >= 0 - (n-1)C
                let dual_v = dual_dist[t].1 - dual_dist[v].1;
                dual_dist[v].0 -= dual_v;
            }

            let mut c = flow_limit - flow;
            let mut v = t;
            while v != s {
                c = c.min(elist[elist[prev_e[v]].rev].cap);
                v = elist[prev_e[v]].to;
            }
            let mut v = t;
            while v != s {
                elist[prev_e[v]].cap += c;
                let elist_prev_e_v_rev = elist[prev_e[v]].rev;
                elist[elist_prev_e_v_rev].cap -= c;
                v = elist[prev_e[v]].to;
            }
            let d = -dual_dist[s].0;
            flow += c;
            cost += c * d;
            if prev_cost_per_flow == d {
                result.pop();
            }
            result.push((flow, cost));
            prev_cost_per_flow = d;
        }

        for i in 0..m {
            let e = &elist[edge_idx[i]];
            self.edges[i].flow = self.edges[i].cap - e.cap;
        }
        result
    }

    fn make_csr(&self, edge_idx: &mut Vec<usize>) -> (Vec<usize>, Vec<_Edge<T>>) {
        let m = self.edges.len();
        let mut degree = vec![0; self.n];
        let mut redge_idx = vec![0; m];
        let mut elist = Vec::new();
        for i in 0..m {
            let e = &self.edges[i];
            edge_idx[i] = degree[e.from];
            degree[e.from] += 1;
            redge_idx[i] = degree[e.to];
            degree[e.to] += 1;
            elist.push((e.from, _Edge {
                to: e.to,
                rev: usize::MAX,
                cap: e.cap - e.flow,
                cost: e.cost,
            }));
            elist.push((e.to, _Edge {
                to: e.from,
                rev: usize::MAX,
                cap: e.flow,
                cost: -e.cost,
            }));
        }
        let mut g = csr(self.n, &elist);
        for i in 0..m {
            let e = &self.edges[i];
            edge_idx[i] += g.0[e.from];
            redge_idx[i] += g.0[e.to];
            g.1[edge_idx[i]].rev = redge_idx[i];
            g.1[redge_idx[i]].rev = edge_idx[i];
        }
        g
    }
}
