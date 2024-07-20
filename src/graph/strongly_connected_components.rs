pub struct StronglyConnectedComponents {
    n: usize,
    edges: Vec<(usize, usize)>,
}

impl StronglyConnectedComponents {
    pub fn new(n: usize) -> Self {
        StronglyConnectedComponents {
            n,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }

    fn csr(&self) -> (Vec<usize>, Vec<usize>) {
        let mut start = vec![0; self.n + 1];
        for (u, _) in self.edges.iter() {
            start[u + 1] += 1;
        }
        for i in 0..self.n {
            start[i + 1] += start[i];
        }
        let mut counter = start.clone();
        let mut elist = vec![0; self.edges.len()];
        for &(u, v) in self.edges.iter() {
            elist[counter[u]] = v;
            counter[u] += 1;
        }
        (start, elist)
    }

    // @return pair of (# of scc, scc id)
    pub fn scc_ids(&self) -> (usize, Vec<usize>) {
        struct Environment {
            n: usize,
            start: Vec<usize>,
            elist: Vec<usize>,
            now_ord: usize,
            group_num: usize,
            visited: Vec<usize>,
            low: Vec<usize>,
            ord: Vec<usize>,
            ids: Vec<usize>,
        }
        let (start, elist) = self.csr();
        let mut env = Environment {
            n: self.n,
            start,
            elist,
            now_ord: 0,
            group_num: 0,
            visited: Vec::with_capacity(self.n),
            low: vec![0; self.n],
            ord: vec![usize::MAX; self.n],
            ids: vec![0; self.n],
        };
        fn dfs(v: usize, env: &mut Environment) {
            env.low[v] = env.now_ord;
            env.ord[v] = env.now_ord;
            env.now_ord += 1;
            env.visited.push(v);
            for i in env.start[v]..env.start[v + 1] {
                let to = env.elist[i];
                if env.ord[to] == usize::MAX {
                    dfs(to, env);
                    env.low[v] = env.low[v].min(env.low[to]);
                } else {
                    env.low[v] = env.low[v].min(env.ord[to]);
                }
            }
            if env.low[v] == env.ord[v] {
                loop {
                    let u = env.visited.pop().unwrap();
                    env.ord[u] = env.n;
                    env.ids[u] = env.group_num;
                    if u == v {
                        break;
                    }
                }
                env.group_num += 1;
            }
        }
        for i in 0..self.n {
            if env.ord[i] == usize::MAX {
                dfs(i, &mut env);
            }
        }
        for x in env.ids.iter_mut() {
            *x = env.group_num - 1 - *x;
        }
        (env.group_num, env.ids)
    }

    pub fn scc(&self) -> Vec<Vec<usize>> {
        let (group_num, ids) = self.scc_ids();
        let mut counts = vec![0; group_num];
        for &x in ids.iter() {
            counts[x] += 1;
        }
        let mut groups = vec![vec![]; group_num];
        for i in 0..group_num {
            groups[i].reserve(counts[i]);
        }
        for i in 0..self.n {
            groups[ids[i]].push(i);
        }
        groups
    }
}

