pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

pub struct SegmentTree<M: Monoid> {
    n: usize,
    size: usize,
    log: usize,
    d: Vec<M::S>,
}

impl <M: Monoid> SegmentTree<M> {
    pub fn new(n: usize) -> Self {
        Self::from_vec(&vec![M::e(); n])
    }

    pub fn from_vec(v: &[M::S]) -> Self {
        let n = v.len();
        let size = n.next_power_of_two();
        let log = n.trailing_zeros() as usize;
        let d = vec![M::e(); 2 * size];
        let mut ret = SegmentTree {
            n,
            log,
            size,
            d,
        };
        for i in 0..n {
            ret.d[size + i] = v[i].clone();
        }
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }

    pub fn set(&mut self, mut p: usize, x: &M::S) {
        debug_assert!(p < self.n);
        p += self.size;
        self.d[p] = x.clone();
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }
    
    pub fn get(&self, p: usize) -> &M::S {
        debug_assert!(p < self.n);
        &self.d[p + self.size]
    }

    pub fn prod(&self, mut l: usize, mut r: usize) -> M::S {
        debug_assert!(l <= r && r <= self.n);
        let mut sml = M::e();
        let mut smr = M::e();
        l += self.size;
        r += self.size;

        while l < r {
            if (l & 1) != 0 {
                sml = M::op(&sml, &self.d[l]);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                smr = M::op(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&sml, &smr)
    }

    pub fn all_prod(&self) -> &M::S {
        &self.d[1]
    }

    pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        debug_assert!(l <= self.n);
        debug_assert!(f(&M::e()));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        let mut sm = M::e();
        while {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f(&M::op(&sm, &self.d[l])) {
                while l < self.size {
                    l = 2 * l;
                    if f(&M::op(&sm, &self.d[l])) {
                        sm = M::op(&sm, &self.d[l]);
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = M::op(&sm, &self.d[l]);
            l += 1;
            (l & (!l + 1)) != l
        } {}
        self.n
    }

    pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        debug_assert!(r <= self.n);
        debug_assert!(f(&M::e()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        let mut sm = M::e();
        while {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !f(&M::op(&self.d[r], &sm)) {
                while r < self.size {
                    r = 2 * r + 1;
                    if f(&M::op(&self.d[r], &sm)) {
                        sm = M::op(&self.d[r], &sm);
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = M::op(&self.d[r], &sm);
            r & (!r + 1) != r
        } {}
        0
    }

    fn update(&mut self, k: usize) {
        self.d[k] = M::op(&self.d[2 * k], &self.d[2 * k + 1]);
    }
}

// struct MinMonoid;

// impl Monoid for MinMonoid {
//     type S = i32;

//     fn e() -> Self::S {
//         Self::S::MAX
//     }

//     fn op(a: &Self::S, b: &Self::S) -> Self::S {
//         *a.min(b)
//     }
// }
