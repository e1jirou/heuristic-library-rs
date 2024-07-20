use crate::graph::strongly_connected_components::StronglyConnectedComponents;

pub struct TwoSat {
    n: usize,
    answer: Vec<bool>,
    scc: StronglyConnectedComponents,
}

impl TwoSat {
    pub fn new(n: usize) -> Self {
        TwoSat {
            n,
            answer: vec![false; n],
            scc: StronglyConnectedComponents::new(2 * n),
        }
    }

    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        debug_assert!(i < self.n);
        debug_assert!(j < self.n);
        self.scc.add_edge(2 * i + if f { 0 } else { 1 }, 2 * j + if g { 1 } else { 0 });
        self.scc.add_edge(2 * j + if g { 0 } else { 1 }, 2 * i + if f { 1 } else { 0 });
    }

    pub fn satisfiable(&mut self) -> bool {
        let id = self.scc.scc_ids().1;
        for i in 0..self.n {
            if id[2 * i] == id[2 * i + 1] {
                return false;
            }
            self.answer[i] = id[2 * i] < id[2 * i + 1];
        }
        return true;
    }
}
