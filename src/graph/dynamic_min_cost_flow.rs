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
    potentials: Vec<T>,
    supplies: Vec<T>,
    cost: T,
    que_min: Vec<usize>,
    que: std::collections::BinaryHeap<std::cmp::Reverse<(T, usize)>>,
}

impl<T> DynamicMinCostFlow<T>
where T: num_traits::NumAssign + num_traits::PrimInt + std::ops::Neg<Output = T>,
{
    pub fn new(n: usize) -> Self {
        DynamicMinCostFlow {
            n,
            edges: vec![Vec::new(); n],
            unused_vertices: Vec::new(),
            potentials: vec![T::zero(); n],
            supplies: vec![T::zero(); n],
            cost: T::zero(),
            que_min: Vec::new(),
            que: std::collections::BinaryHeap::new(),
        }
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

    pub fn add_edge(&mut self, from: usize, to: usize, cap: T, cost: T) -> usize {
        debug_assert!(from < self.n);
        debug_assert!(to < self.n);
        debug_assert_ne!(from, to);
        debug_assert!(cap >= T::zero());
        debug_assert!(cost >= T::zero());
        if cost + self.potentials[from] - self.potentials[to] >= T::zero() {
            return self.add_edge(from, to, cap, cost);
        }
        todo!("delete negative cycle");
    }

    pub fn remove_edge(&mut self, from: usize, fwd: usize) {
        let e_fwd = self.edges[from][fwd].clone();
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
            self.potentials.push(T::zero());
            self.supplies.push(T::zero());
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

    // return the amount of the flow and the cost
    pub fn flow() -> (T, T) {
        todo!();
    }
}
