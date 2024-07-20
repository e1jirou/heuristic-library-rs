pub struct FenwickTree<T> {
    n: usize,
    log: usize,
    data: Vec<T>
}

impl<T: num_traits::NumAssign + num_traits::PrimInt> FenwickTree<T> {
    pub fn new(n: usize) -> Self {
        FenwickTree {
            n,
            log: (64 - (n as u64).leading_zeros()) as usize,
            data: vec![T::zero(); n],
        }
    }

    pub fn add(&mut self, mut p: usize, x: T) {
        debug_assert!(p < self.n);
        p += 1;
        while p <= self.n {
            self.data[p - 1] += x;
            p += p & (!p + 1);
        }
    }

    pub fn sum(&self, l: usize, r: usize) -> T {
        debug_assert!(l <= r && r <= self.n);
        self.sum_left(r) - self.sum_left(l)
    }

    pub fn sum_left(&self, mut r: usize) -> T {
        debug_assert!(r <= self.n);
        let mut s = T::zero();
        while r > 0 {
            s += self.data[r - 1];
            r -= r & (!r + 1);
        }
        s
    }

    pub fn lower_bound(&self, mut x: T) -> usize {
        if x <= T::zero() {
            return 0;
        }
        let mut ret = 0;
        for i in (0..self.log).rev() {
            let p = ret | (1 << i);
            if p <= self.n && self.data[p - 1] < x {
                x -= self.data[p - 1];
                ret = p;
            }
        }
        ret
    }

    pub fn min_right(&self, l: usize) -> Option<usize> {
        debug_assert!(l <= self.n);
        let s = self.sum_left(l);
        let i = self.lower_bound(s + T::one());
        if i == self.n {
            None
        } else {
            Some(i)
        }
    }

    pub fn max_left(&self, r: usize) -> Option<usize> {
        debug_assert!(r <= self.n);
        let s = self.sum_left(r);
        if s == T::zero() {
            None
        } else {
            Some(self.lower_bound(s))
        }
    }
}
