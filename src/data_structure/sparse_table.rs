pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

// Constraint: op(x, x) = x
// Example: min, max
pub struct SparseTable<M: Monoid> {
    n: usize,
    table: Vec<M::S>,
}

impl<M: Monoid> SparseTable<M> {
    pub fn from_vec(v: &[M::S]) -> Self {
        debug_assert!(!v.is_empty());
        let n = v.len();
        let log = 64 - (n as u64).leading_zeros() as usize;
        let mut table = v.to_vec();
        table.resize(n * log, M::e());
        for i in 0..(log - 1) {
            let ni = n * i;
            for j in 0..=(n - (1 << (i + 1))) {
                let p = ni + j;
                table[p + n] = M::op(&table[p], &table[p + (1 << i)]);
            }
        }
        SparseTable {
            n,
            table,
        }
    }

    // [l, r)
    pub fn prod(&self, l: usize, r: usize) -> M::S {
        debug_assert!(l <= r && r <= self.n);
        if l == r {
            return M::e();
        }
        let log = 63 - ((r - l) as usize).leading_zeros() as usize;
        M::op(&self.table[self.n * log + l], &self.table[self.n * log + r - (1 << log)])
    }
}

// struct MinMonoid;

// impl Monoid for MinMonoid {
//     type S = u32;

//     fn e() -> Self::S {
//         Self::S::MAX
//     }

//     fn op(a: &Self::S, b: &Self::S) -> Self::S {
//         *a.min(b)
//     }
// }
