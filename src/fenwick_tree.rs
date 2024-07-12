type T = i32;

pub struct FenwickTree {
    n: usize,
    log: usize,
    data: Vec<T>
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        FenwickTree {
            n,
            log: (64 - (n as u64).leading_zeros()) as usize,
            data: vec![T::default(); n],
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
        let mut s = 0;
        while r > 0 {
            s += self.data[r - 1];
            r -= r & (!r + 1);
        }
        s
    }

    pub fn bisect(&self, x: T) -> usize {
        let mut s = 0;
        let mut p = 0;
        for i in (0..self.log).rev() {
            let q = p | (1 << i);
            if q > self.n {
                continue;
            }
            let t = s + self.data[q - 1];
            if t < x {
                s = t;
                p = q;
            }
        }
        p
    }

    pub fn min_right(&self, l: usize) -> Option<usize> {
        debug_assert!(l <= self.n);
        let s = self.sum_left(l);
        let i = self.bisect(s + 1);
        if i == self.n {
            None
        } else {
            Some(i)
        }
    }

    pub fn max_left(&self, r: usize) -> Option<usize> {
        debug_assert!(r <= self.n);
        let s = self.sum_left(r);
        if s == 0 {
            None
        } else {
            Some(self.bisect(s))
        }
    }
}
