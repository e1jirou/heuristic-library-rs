pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

pub trait MapMonoid {
    type M: Monoid;
    type F: Clone;
    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
    fn composition(f: &Self::F, g: &Self::F) -> Self::F;
    fn id() -> Self::F;
}

pub struct LazySegmentTree<T: MapMonoid> {
    n: usize,
    size: usize,
    log: usize,
    d: Vec<<T::M as Monoid>::S>,
    lz: Vec<T::F>,
}

impl <T: MapMonoid> LazySegmentTree<T> {
    pub fn new(n: usize) -> Self {
        Self::from_vec(&vec![T::M::e(); n])
    }

    pub fn from_vec(v: &Vec<<T::M as Monoid>::S>) -> Self {
        let n = v.len();
        let log = (64 - ((n as u64).saturating_sub(1).leading_zeros())) as usize;
        let size = 1 << log;
        let d = vec![T::M::e(); 2 * size];
        let lz = vec![T::id(); size];
        let mut ret = LazySegmentTree {
            n,
            log,
            size,
            d,
            lz,
        };
        for i in 0..n {
            ret.d[size + i] = v[i].clone();
        }
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }

    pub fn set(&mut self, mut p: usize, x: <T::M as Monoid>::S) {
        debug_assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = x.clone();
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get(&mut self, mut p: usize) -> &<T::M as Monoid>::S {
        debug_assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        &self.d[p]
    }

    pub fn prod(&mut self, mut l: usize, mut r: usize) -> <T::M as Monoid>::S {
        debug_assert!(l <= r && r <= self.n);
        if l == r {
            return T::M::e();
        }
        l += self.size;
        r += self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        let mut sml = T::M::e();
        let mut smr = T::M::e();
        while l < r {
            if (l & 1) != 0 {
                sml = T::M::op(&sml, &self.d[l]);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                smr = T::M::op(&self.d[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }
        T::M::op(&sml, &smr)
    }

    pub fn all_prod(&self) -> &<T::M as Monoid>::S {
        &self.d[1]
    }

    pub fn apply<F>(&mut self, mut p: usize, f: &T::F) {
        debug_assert!(p < self.n);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.d[p] = T::mapping(f, &self.d[p]);
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn apply_range(&mut self, mut l: usize, mut r: usize, f: &T::F) {
        debug_assert!(l <= r && r <= self.n);
        if l == r {
            return;
        }
        l += self.size;
        r += self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        {
            let l2 = l;
            let r2 = r;
            while l < r {
                if (l & 1) != 0 {
                    self.all_apply(l, f);
                    l += 1;
                }
                if (r & 1) != 0 {
                    r -= 1;
                    self.all_apply(r, f);
                }
                l >>= 1;
                r >>= 1;
            }
            l = l2;
            r = r2;
        }
        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
    }

    pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize
    where
        G: Fn(&<T::M as Monoid>::S) -> bool,
    {
        debug_assert!(l <= self.n);
        debug_assert!(g(&T::M::e()));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        for i in (1..=self.log).rev() {
            self.push(l >> i);
        }
        let mut sm = T::M::e();
        while {
            while l % 2 == 0 {
                l >>= 1;
            }
            if !g(&T::M::op(&sm, &self.d[l])) {
                while l < self.size {
                    self.push(l);
                    l = 2 * l;
                    if g(&T::M::op(&sm, &self.d[l])) {
                        sm = T::M::op(&sm, &self.d[l]);
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = T::M::op(&sm, &self.d[l]);
            l += 1;
            (l & (!l + 1)) != l
        } {}
        self.n
    }

    pub fn min_left<G>(&mut self, mut r: usize, g: G) -> usize
    where
        G: Fn(&<T::M as Monoid>::S) -> bool,
    {
        debug_assert!(r <= self.n);
        debug_assert!(g(&T::M::e()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        for i in (1..=self.log).rev() {
            self.push((r - 1) >> i);
        }
        let mut sm = T::M::e();
        while {
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !g(&T::M::op(&self.d[r], &sm)) {
                while r < self.size {
                    self.push(r);
                    r = 2 * r + 1;
                    if g(&T::M::op(&self.d[r], &sm)) {
                        sm = T::M::op(&self.d[r], &sm);
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = T::M::op(&self.d[r], &sm);
            r & (!r + 1) != r
        } {}
        0
    }

    fn update(&mut self, k: usize) {
        self.d[k] = T::M::op(&self.d[2 * k], &self.d[2 * k + 1]);
    }

    fn all_apply(&mut self, k: usize, f: &T::F) {
        self.d[k] = T::mapping(&f, &self.d[k]);
        if k < self.size {
            self.lz[k] = T::composition(&f, &self.lz[k]);
        }
    }

    fn push(&mut self, k: usize) {
        self.all_apply(2 * k, &self.lz[k].clone());
        self.all_apply(2 * k + 1, &self.lz[k].clone());
        self.lz[k] = T::id();
    }
}

// const MOD: i64 = 998244353;

// struct SumMonoid;

// impl Monoid for SumMonoid {
//     type S = (i64, i64);

//     fn e() -> Self::S {
//         (0, 0)
//     }

//     fn op(a: &Self::S, b: &Self::S) -> Self::S {
//         let mut s = a.0 + b.0;
//         if s >= MOD {
//             s -= MOD;
//         }
//         (s, a.1 + b.1)
//     }
// }

// struct AffineMapMonoid;

// impl MapMonoid for AffineMapMonoid {
//     type M = SumMonoid;
//     type F = (i64, i64);

//     fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
//         ((f.0 * x.0 + f.1 * x.1) % MOD, x.1)
//     }

//     fn composition(f: &Self::F, g: &Self::F) -> Self::F {
//         (f.0 * g.0 % MOD, (f.0 * g.1 + f.1) % MOD)
//     }

//     fn id() -> Self::F {
//         (1, 0)
//     }
// }
