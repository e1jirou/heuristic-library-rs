pub struct FenwickTree2D<T> {
    n: usize,
    m: usize,
    data: Vec<T>,
}

impl<T: num_traits::NumAssign + num_traits::PrimInt + std::ops::Neg<Output = T>> FenwickTree2D<T> {
    pub fn new(n: usize, m: usize) -> Self {
        FenwickTree2D {
            n,
            m,
            data: vec![T::zero(); (n + 1) * (m + 1)],
        }
    }

    pub fn add(&mut self, p: usize, q: usize, x: T) {
        debug_assert!(p <= self.n);
        debug_assert!(q <= self.m);
        let mut i = p + 1;
        while i <= self.n {
            let offset = (self.m + 1) * (i - 1);
            let mut j = q + 1;
            while j <= self.m {
                self.data[offset + j - 1] += x;
                j += j & (!j + 1);
            }
            i += i & (!i + 1);
        }
    }

    // add [pl, pr) x [ql, qr) for imos method
    pub fn rectangle_add(&mut self, pl: usize, pr: usize, ql: usize, qr: usize, x: T) {
        debug_assert!(pl <= pr && pr <= self.n);
        debug_assert!(ql <= qr && qr <= self.m);
        self.add(pl, ql, x);
        self.add(pl, qr, -x);
        self.add(pr, ql, -x);
        self.add(pr, qr, x);
    }

    // [0, p) x [0, q)
    pub fn sum_left(&self, p: usize, q: usize) -> T {
        debug_assert!(p <= self.n);
        debug_assert!(q <= self.m);
        let mut s = T::zero();
        let mut i = p;
        while i > 0 {
            let offset = (self.m + 1) * (i - 1);
            let mut j = q;
            while j > 0 {
                s += self.data[offset + j - 1];
                j -= j & (!j + 1);
            }
            i -= i & (!i + 1);
        }
        s
    }

    // [pl, pr) x [ql, qr)
    pub fn sum(&self, pl: usize, pr: usize, ql: usize, qr: usize) -> T {
        debug_assert!(pl <= pr && pr <= self.n);
        debug_assert!(ql <= qr && qr <= self.m);
        self.sum_left(pr, qr) + self.sum_left(pl, ql) - self.sum_left(pl, qr) - self.sum_left(pr, ql)
    }
}
