pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

struct SlidingWindowAggregation<M: Monoid> {
    buf: Vec<M::S>,
    former: Vec<M::S>,
    latter: Vec<M::S>,
}

impl<M: Monoid> SlidingWindowAggregation<M> {
    pub fn new() -> Self {
        SlidingWindowAggregation {
            buf: Vec::new(),
            former: vec![M::e()],
            latter: vec![M::e()],
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut ret = Self::new();
        ret.buf.reserve(capacity);
        ret.former.reserve(capacity);
        ret.latter.reserve(capacity);
        ret
    }

    pub fn len(&self) -> usize {
        self.former.len() + self.latter.len() - 2
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn all_prod(&self) -> M::S {
        M::op(self.former.last().unwrap(), self.latter.last().unwrap())
    }

    // push back
    pub fn push(&mut self, x: M::S) {
        self.latter.push(M::op(self.latter.last().unwrap(), &x));
        self.buf.push(x);
    }

    // pop front
    pub fn pop(&mut self) {
        if self.former.len() == 1 {
            self.reconstruct();
        } else {
            self.former.pop();
        }
    }

    fn reconstruct(&mut self) {
        for x in self.buf[1..].iter().rev() {
            self.former.push(M::op(x, self.former.last().unwrap()));
        }
        self.buf.clear();
        self.latter.truncate(1);
    }
}

// const MOD: i64 = 998_244_353;

// struct AffineMonoid;

// impl Monoid for AffineMonoid {
//     type S = (i64, i64);

//     fn e() -> Self::S {
//         (1, 0)
//     }

//     fn op(a: &Self::S, b: &Self::S) -> Self::S {
//         (b.0 * a.0 % MOD, (b.1 + b.0 * a.1) % MOD)
//     }
// }
