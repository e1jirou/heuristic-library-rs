pub struct LowLinkForUndirectedGraph {
    edges: Vec<Vec<usize>>,
    ord: Vec<usize>,
    low: Vec<usize>,
    is_articulation_point: Vec<bool>,
}

impl LowLinkForUndirectedGraph {
    pub fn new(edges: Vec<Vec<usize>>) -> Self {
        let n = edges.len();
        let mut ret = LowLinkForUndirectedGraph {
            edges,
            ord: vec![usize::MAX; n],
            low: vec![usize::MAX; n],
            is_articulation_point: vec![false; n],
        };
        ret.low_link(0, 0, usize::MAX);
        ret
    }

    fn low_link(&mut self, v: usize, mut i: usize, parent: usize) -> usize {
        debug_assert_eq!(self.ord[v], usize::MAX);
        self.ord[v] = i;
        i += 1;
        let mut num_children = 0;
        for edges_v_id in 0..self.edges[v].len() {
            let u = self.edges[v][edges_v_id];
            if self.ord[u] == usize::MAX {
                // forward edge
                num_children += 1;
                i = self.low_link(u, i, v);
                self.low[v] = self.low[v].min(self.low[u]);
                if self.ord[v] > 0 && self.ord[v] <= self.low[u] {
                    self.is_articulation_point[v] = true;
                }
            } else {
                // backward edge
                if u != parent {
                    self.low[v] = self.low[v].min(self.ord[u]);
                }
            }
        }
        if self.ord[v] == 0 && num_children >= 2 {
            // root is an articulation point
            self.is_articulation_point[v] = true;
        }
        i
    }
}

// This function aids in making a conservative assessment of
// whether a vertex within a grid is an articulation point,
// considering only the eight surrounding vertices.
// 0 1 2
// 3 4 5
// 6 7 8
pub fn make_connected3x3() -> fixedbitset::FixedBitSet {
    let mut edges = vec![Vec::new(); 9];
    for v in 0..9 {
        if v / 3 > 0 {
            edges[v].push(v - 3);
        }
        if v % 3 > 0 {
            edges[v].push(v - 1);
        }
        if v % 3 < 2 {
            edges[v].push(v + 1);
        }
        if v / 3 < 2{
            edges[v].push(v + 3);
        }
    }
    let mut connected3x3 = fixedbitset::FixedBitSet::with_capacity(512);
    for s in 1..512 as usize {
        // Depth First Search
        let root = s.trailing_zeros();
        let mut visited = 1 << root;
        let mut todo: usize = 1 << root;
        while todo > 0 {
            let u = todo.trailing_zeros() as usize;
            todo ^= 1 << u;
            for &v in &edges[u] {
                if (s & (1 << v)) > 0 && (visited & (1 << v)) == 0 {
                    visited |= 1 << v;
                    todo |= 1 << v;
                }
            }
        }
        if visited == s {
            connected3x3.set(s, true);
        }
    }
    connected3x3
}
