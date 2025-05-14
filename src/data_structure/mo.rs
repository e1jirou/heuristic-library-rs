struct Mo {
    n: usize,
    q: usize,
    b: usize, // block size
    offset: usize,
    blocks: Vec<Vec<(u32, u32, u32)>>, // (l, r, i)
}

impl Mo {
    fn new(n: usize, q: usize, offset: bool) -> Self {
        let b = (n as f64 / (2.0 * (q + 1) as f64).sqrt()).ceil() as usize;
        let offset = if offset { b / 2 } else { 0 };
        Self {
            n,
            q,
            b,
            offset,
            blocks: vec![vec![]; (n + offset) / b + 1],
        }
    }

    fn add(&mut self, l: usize, r: usize, i: usize) {
        debug_assert!(l <= r);
        debug_assert!(r <= self.n);
        debug_assert!(i < self.q);
        let block = (l + self.offset) / self.b;
        self.blocks[block].push((l as u32, r as u32, i as u32));
    }

    fn build(&mut self) {
        self.sort();
    }

    fn evaluate(&self) -> u32 {
        let mut cost = 0;
        let mut l0 = 0u32;
        let mut r0 = 0u32;
        for block in &self.blocks {
            for &(l1, r1, _) in block {
                cost += l0.abs_diff(l1) + r0.abs_diff(r1);
                l0 = l1;
                r0 = r1;
            }
        }
        cost
    }

    fn order(&mut self) -> Vec<(usize, usize, usize)> {
        self.blocks
            .iter()
            .flat_map(|block| {
                block
                    .iter()
                    .map(move |&(l, r, i)| (l as usize, r as usize, i as usize))
            })
            .collect()
    }

    fn sort(&mut self) {
        let mut l_last = 0u32;
        let mut r_last = 0;
        for block in &mut self.blocks {
            let block_len = block.len();
            if block_len <= 1 {
                continue;
            }
            block.sort_unstable_by_key(|&(_, r, _)| r);
            if block[0].1.abs_diff(r_last) > block[block_len - 1].1.abs_diff(r_last) {
                block.reverse();
            }
            // improve by swap
            let mut l0 = l_last;
            let mut r0 = r_last;
            let mut l1 = block[0].0;
            let mut r1 = block[0].1;
            let mut l2 = block[1].0;
            let mut r2 = block[1].1;
            for i in 2..block_len {
                let l3 = block[i].0;
                let r3 = block[i].1;
                let mut profit = (l0.abs_diff(l1) + r0.abs_diff(r1)) as i32;
                profit += (l2.abs_diff(l3) + r2.abs_diff(r3)) as i32;
                profit -= (l0.abs_diff(l2) + r0.abs_diff(r2)) as i32;
                profit -= (l1.abs_diff(l3) + r1.abs_diff(r3)) as i32;
                if profit > 0 {
                    block.swap(i - 2, i - 1);
                    l0 = l2;
                    r0 = r2;
                } else {
                    l0 = l1;
                    r0 = r1;
                    l1 = l2;
                    r1 = r2;
                }
                l2 = l3;
                r2 = r3;
            }
            l_last = block[block_len - 1].0;
            r_last = block[block_len - 1].1;
        }
    }
}

pub struct Mos {
    mos: Vec<Mo>,
}

impl Mos {
    pub fn new(n: usize, q: usize) -> Self {
        let mut mos = vec![];
        mos.push(Mo::new(n, q, false));
        mos.push(Mo::new(n, q, true));
        Self { mos }
    }

    pub fn add(&mut self, l: usize, r: usize, i: usize) {
        for mo in &mut self.mos {
            mo.add(l, r, i);
        }
    }

    pub fn build(&mut self) -> Vec<(usize, usize, usize)> {
        for mo in &mut self.mos {
            mo.build();
        }
        let mut best = 0;
        let mut min_cost = u32::MAX;
        for i in 0..self.mos.len() {
            let cost = self.mos[i].evaluate();
            if cost < min_cost {
                min_cost = cost;
                best = i;
            }
        }
        self.mos[best].order()
    }
}
