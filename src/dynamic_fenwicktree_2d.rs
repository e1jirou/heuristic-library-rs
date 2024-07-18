use crate::dynamic_fenwick_tree::DynamicFenwickTree;

pub struct DynamicFenwickTree2D<T> {
    n: usize,
    m: usize,
    data: Vec<DynamicFenwickTree<T>>,
}

impl<T: num_traits::NumAssign + num_traits::PrimInt + std::ops::Neg<Output = T>> DynamicFenwickTree2D<T> {
    pub fn new(n: usize, m: usize) -> Self {
        DynamicFenwickTree2D {
            n,
            m,
            data: vec![DynamicFenwickTree::new(m); n],
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    pub fn add(&mut self, mut p: usize, q: usize, x: T) {
        debug_assert!(p < self.n);
        debug_assert!(q < self.m);
        p += 1;
        while p <= self.n {
            self.data[p - 1].add(q, x);
            p += p & (!p + 1);
        }
    }

    // add [pl, pr) x [ql, qr) for imos method
    pub fn rectangle_add(&mut self, pl: usize, ql: usize, pr: usize, qr: usize, x: T) {
        debug_assert!(pl <= pr && pr <= self.n);
        debug_assert!(ql <= qr && qr <= self.m);
        let mut add_if_valid = |p, q, x| {
            if p < self.n && q < self.m {
                self.add(p, q, x);
            }
        };
        add_if_valid(pl, ql, x);
        add_if_valid(pl, qr, -x);
        add_if_valid(pr, ql, -x);
        add_if_valid(pr, qr, x);
    }

    // [0, p) x [0, q)
    pub fn sum_left(&self, mut p: usize, q: usize) -> T {
        debug_assert!(p <= self.n);
        debug_assert!(q <= self.m);
        let mut s = T::zero();
        while p > 0 {
            s += self.data[p - 1].sum_left(q);
            p -= p & (!p + 1);
        }
        s
    }

    // [pl, pr) x [ql, qr)
    pub fn sum(&self, mut pl: usize, ql: usize, mut pr: usize, qr: usize) -> T {
        debug_assert!(pl <= pr && pr <= self.n);
        debug_assert!(ql <= qr && qr <= self.m);
        let mut s = T::zero();
        while pl != pr {
            if pl < pr {
                s += self.data[pr - 1].sum(ql, qr);
                pr -= pr & (!pr + 1);
            } else {
                s -= self.data[pl - 1].sum(ql, qr);
                pl -= pl & (!pl + 1);
            }
        }
        s
    }
}
