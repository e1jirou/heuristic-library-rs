#[derive(Debug, Clone)]
pub struct FenwickTree<T> {
    n: usize,
    log: usize,
    data: Vec<T>
}

impl<T: num_traits::NumAssign + num_traits::PrimInt> FenwickTree<T> {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            log: (64 - (n as u64).leading_zeros()) as usize,
            data: vec![T::zero(); n],
        }
    }

    pub fn from_vec(mut data: Vec<T>) -> Self {
        let n = data.len();
        for i in 0..n {
            let j = i | (i + 1);
            if j < n {
                data[j] = data[i] + data[j];
            }
        }
        Self {
            n,
            log: (64 - (n as u64).leading_zeros()) as usize,
            data,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec_sum_queries() {
        let fw = FenwickTree::from_vec(vec![3_i64, 1, 4, 1, 5]);

        assert_eq!(fw.sum_left(0), 0);
        assert_eq!(fw.sum_left(1), 3);
        assert_eq!(fw.sum_left(3), 8);
        assert_eq!(fw.sum_left(5), 14);

        assert_eq!(fw.sum(0, 5), 14);
        assert_eq!(fw.sum(1, 4), 6);
        assert_eq!(fw.sum(2, 2), 0);
        assert_eq!(fw.sum(3, 5), 6);
    }

    #[test]
    fn test_add_after_from_vec() {
        let mut fw = FenwickTree::from_vec(vec![2_i64, 0, 1, 3]);

        fw.add(1, 5);
        fw.add(3, -2);

        assert_eq!(fw.sum_left(4), 9);
        assert_eq!(fw.sum(0, 2), 7);
        assert_eq!(fw.sum(1, 4), 7);
        assert_eq!(fw.sum(3, 4), 1);
    }

    #[test]
    fn test_lower_bound_min_right_max_left() {
        let fw = FenwickTree::from_vec(vec![0_i64, 2, 0, 3, 0]);

        assert_eq!(fw.lower_bound(-5), 0);
        assert_eq!(fw.lower_bound(0), 0);
        assert_eq!(fw.lower_bound(1), 1);
        assert_eq!(fw.lower_bound(2), 1);
        assert_eq!(fw.lower_bound(3), 3);
        assert_eq!(fw.lower_bound(5), 3);
        assert_eq!(fw.lower_bound(6), 5);

        assert_eq!(fw.min_right(0), Some(1));
        assert_eq!(fw.min_right(1), Some(1));
        assert_eq!(fw.min_right(2), Some(3));
        assert_eq!(fw.min_right(3), Some(3));
        assert_eq!(fw.min_right(4), None);
        assert_eq!(fw.min_right(5), None);

        assert_eq!(fw.max_left(0), None);
        assert_eq!(fw.max_left(1), None);
        assert_eq!(fw.max_left(2), Some(1));
        assert_eq!(fw.max_left(3), Some(1));
        assert_eq!(fw.max_left(4), Some(3));
        assert_eq!(fw.max_left(5), Some(3));
    }
}
