pub fn next_permutation<T: Ord>(p: &mut [T]) -> bool {
    let n = p.len();
    if n <= 1 {
        return false;
    }
    for i in (0..(n - 1)).rev() {
        if p[i] < p[i + 1] {
            for j in (i..n).rev() {
                if p[i] >= p[j] {
                    continue;
                }
                p.swap(i, j);
                p[(i + 1)..].reverse();
                return true;
            }
        }
    }
    false
}

pub struct Combinations<T> {
    n: usize,
    r: usize,
    data: Vec<T>,
    indices: Vec<usize>,
    combination: Vec<T>,
}

impl<T: Clone> Combinations<T> {
    pub fn new(data: &[T], r: usize) -> Self {
        let n = data.len();
        debug_assert!(r <= n);
        Combinations {
            n,
            r,
            data: data.to_vec(),
            indices: (0..r).collect(),
            combination: data[0..r].to_vec(),
        }
    }

    pub fn get_combination(&self) -> &Vec<T> {
        &self.combination
    }

    pub fn next_combination(&mut self) -> bool {
        // backward
        while !self.indices.is_empty() && *self.indices.last().unwrap() == self.n - self.r + self.indices.len() - 1 {
            self.indices.pop();
            self.combination.pop();
        }
        let s = self.indices.len();
        if s == 0 {
            return false;
        }
        // forward
        let mut p = self.indices[s - 1];
        self.indices.pop();
        self.combination.pop();
        for _ in (s - 1)..self.r {
            p += 1;
            self.indices.push(p);
            self.combination.push(self.data[p].clone());
        }
        true
    }
}

pub struct CombinationsWithReplacement<T> {
    n: usize,
    r: usize,
    data: Vec<T>,
    indices: Vec<usize>,
    combination: Vec<T>,
}

impl<T: Clone> CombinationsWithReplacement<T> {
    pub fn new(data: &[T], r: usize) -> Self {
        let n = data.len();
        debug_assert!(r <= n);
        CombinationsWithReplacement {
            n,
            r,
            data: data.to_vec(),
            indices: vec![0; r],
            combination: vec![data[0].clone(); r],
        }
    }

    pub fn get_combination(&self) -> &Vec<T> {
        &self.combination
    }

    pub fn next_combination(&mut self) -> bool {
        // backward
        while !self.indices.is_empty() && *self.indices.last().unwrap() == self.n - 1 {
            self.indices.pop();
            self.combination.pop();
        }
        let s = self.indices.len();
        if s == 0 {
            return false;
        }
        // forward
        let p = self.indices[s - 1] + 1;
        self.indices.pop();
        self.combination.pop();
        for _ in (s - 1)..self.r {
            self.indices.push(p);
            self.combination.push(self.data[p].clone());
        }
        true
    }
}

pub fn pop_one<T: num_traits::PrimInt + std::ops::BitXorAssign>(x: &mut T) -> Option<u32> {
    if *x == T::zero() {
        return None;
    }
    let ret = x.trailing_zeros();
    *x ^= T::one() << ret as usize;
    Some(ret)
}
