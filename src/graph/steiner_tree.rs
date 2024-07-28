pub struct SteinerTree<Cost> {
    n: usize,
    edges: Vec<Vec<(usize, Cost)>>,
    dp: Vec<Vec<Cost>>,
}

impl<Cost: num_traits::NumAssign + num_traits::PrimInt> SteinerTree<Cost> {
    pub fn new(n: usize) -> Self {
        SteinerTree {
            n,
            edges: vec![Vec::new(); n],
            dp: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, cost: Cost) {
        debug_assert!(u < self.n);
        debug_assert!(v < self.n);
        self.edges[u].push((v, cost));
        self.edges[v].push((u, cost));
    }

    pub fn solve(&mut self, terminals: &[usize]) -> Cost {
        if terminals.is_empty() {
            return Cost::zero();
        }
        let inf_cost = Cost::max_value() >> 1;
        self.dp = vec![vec![inf_cost; self.n]; 1 << terminals.len()];
        for i in 0..terminals.len() {
            self.dp[1 << i][terminals[i]] = Cost::zero();
        }
        let mut que = std::collections::BinaryHeap::with_capacity(self.n);
        for s in 1..(1 << terminals.len()) {
            for v in 0..self.n {
                let mut t = s;
                while t > 0 {
                    self.dp[s][v] = self.dp[s][v].min(self.dp[t][v] + self.dp[s ^ t][v]);
                    t = (t - 1) & s;
                }
            }
            for v in 0..self.n {
                que.push(std::cmp::Reverse((self.dp[s][v], v)));
            }
            while let Some(std::cmp::Reverse((cost, u))) = que.pop() {
                if self.dp[s][u] < cost {
                    continue;
                }
                for &(v, mut c) in &self.edges[u] {
                    c += cost;
                    if c < self.dp[s][v] {
                        self.dp[s][v] = c;
                        que.push(std::cmp::Reverse((c, v)));
                    }
                }
            }
        }
        self.dp[(1 << terminals.len()) - 1][terminals[0]]
    }
}
