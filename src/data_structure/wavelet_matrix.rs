#[derive(Clone)]
struct BitVector {
    n: usize,
    zeros: usize,
    block: Vec<u64>,
    count: Vec<usize>,
}

impl BitVector {
    pub fn new(n: usize) -> Self {
        BitVector {
            n,
            zeros: 0,
            block: vec![0; n / 64 + 1],
            count: Vec::new(),
        }
    }

    // do not call set after build
    pub fn set(&mut self, p: usize) {
        debug_assert!(p < self.n);
        self.block[p / 64] |= 1 << (p % 64);
    }

    pub fn get(&self, p: usize) -> bool {
        debug_assert!(p < self.n);
        ((self.block[p / 64] >> (p % 64)) & 1) == 1
    }

    pub fn build(&mut self) {
        self.count = vec![0; self.block.len()];
        for i in 1..self.block.len() {
            self.count[i] = self.count[i - 1] + self.block[i - 1].count_ones() as usize;
        }
        self.zeros = self.rank0(self.n);
    }

    pub fn rank0(&self, p: usize) -> usize {
        debug_assert!(p <= self.n);
        p - self.rank1(p)
    }

    pub fn rank1(&self, p: usize) -> usize {
        debug_assert!(p <= self.n);
        self.count[p / 64] + (self.block[p / 64] & ((1 << (p % 64)) - 1)).count_ones() as usize
    }
}

pub struct WaveletMatrix<T> {
    n: usize,
    log: usize,
    #[allow(unused)]
    data: Vec<T>,
    bv: Vec<BitVector>,
}

impl<T: num_traits::PrimInt + std::ops::BitOrAssign> WaveletMatrix<T> {
    pub fn from_vec(data: &[T]) -> Self {
        debug_assert!(!data.is_empty());
        let n = data.len();
        let log = 8 * std::mem::size_of::<T>() - data.iter().map(|x| x.leading_zeros()).min().unwrap() as usize + 1;
        let mut bv = vec![BitVector::new(n); log];
        let mut curr_data = data.to_vec();
        let mut next_data = vec![T::zero(); n];
        for h in (0..log).rev() {
            for i in 0..n {
                if ((curr_data[i] >> h) & T::one()) == T::one() {
                    bv[h].set(i);
                }
            }
            bv[h].build();
            let mut zero_index = 0;
            let mut one_index = bv[h].zeros;
            for i in 0..n {
                if bv[h].get(i) {
                    next_data[one_index] = curr_data[i];
                    one_index += 1;
                } else {
                    next_data[zero_index] = curr_data[i];
                    zero_index += 1;
                }
            }
            std::mem::swap(&mut curr_data, &mut next_data);
        }
        WaveletMatrix {
            n,
            log,
            data: data.to_vec(),
            bv,
        }
    }

    pub fn kth_smallest(&self, mut l: usize, mut r: usize, mut k: usize) -> T {
        debug_assert!(l <= r && r <= self.n);
        debug_assert!(k < r - l);
        let mut ret = T::zero();
        for h in (0..self.log).rev() {
            let l0 = self.bv[h].rank0(l);
            let r0 = self.bv[h].rank0(r);
            if k < r0 - l0 {
                l = l0;
                r = r0;
            } else {
                k -= r0 - l0;
                ret |= T::one() << h;
                l += self.bv[h].zeros - l0;
                r += self.bv[h].zeros - r0;
            }
        }
        ret
    }

    pub fn kth_largest(&self, l: usize, r: usize, k: usize) -> T {
        debug_assert!(l <= r && r <= self.n);
        debug_assert!(k < r - l);
        self.kth_smallest(l, r, r - l - k - 1)
    }

    pub fn range_freq(&self, mut l: usize, mut r: usize, upper: T) -> usize {
        debug_assert!(l <= r && r <= self.n);
        if upper >= T::one() << self.log {
            return r - l;
        }
        let mut ret = 0;
        for h in (0..self.log).rev() {
            let l0 = self.bv[h].rank0(l);
            let r0 = self.bv[h].rank0(r);
            if ((upper >> h) & T::one()) == T::one()  {
                ret += r0 - l0;
                l += self.bv[h].zeros - l0;
                r += self.bv[h].zeros - r0;
            } else {
                l = l0;
                r = r0;
            }
        }
        ret
    }

    pub fn range_freq_range(&self, l: usize, r: usize, lower: T, upper: T) -> usize {
        debug_assert!(l <= r && r <= self.n);
        self.range_freq(l, r, upper) - self.range_freq(l, r, lower)
    }
}
