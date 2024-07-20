#[derive(Clone)]
pub struct DynamicFenwickTree<T> {
    n: usize,
    log: usize,
    data: std::collections::HashMap<usize, T>,
}

impl<T: num_traits::NumAssign + num_traits::PrimInt> DynamicFenwickTree<T> {
    pub fn new(n: usize) -> Self {
        DynamicFenwickTree {
            n,
            log: (64 - (n as u64).leading_zeros()) as usize,
            data: std::collections::HashMap::new(),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    pub fn add(&mut self, mut p: usize, x: T) {
        debug_assert!(p < self.n);
        p += 1;
        while p <= self.n {
            match self.data.get_mut(&(p - 1)) {
                Some(y) => {
                    *y += x;
                }
                None => {
                    self.data.insert(p - 1, x);
                }
            }
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
            if let Some(&x) = self.data.get(&(r - 1)) {
                s += x;
            }
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
            if p <= self.n {
                match self.data.get(&(p - 1)) {
                    Some(&y) =>  {
                        if y < x {
                            x -= y;
                            ret = p;
                        }
                    }
                    None => {
                        ret = p;
                    }
                }
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
