struct Hungarian<T> {
    n: usize,
    matrix: Vec<T>,
    zeros: Vec<usize>,
    used_row: Vec<bool>,
    used_col: Vec<bool>,

}

impl <T: num_traits::NumAssign + num_traits::PrimInt> Hungarian<T> {
    pub fn new(n: usize, matrix: Vec<T>) -> Self {
        debug_assert!(matrix.len() == n * n);
        Hungarian {
            n,
            matrix,
            zeros: Vec::with_capacity(n * n),
            used_row: vec![false; n],
            used_col: vec![false; n],
        }
    }

    // return minimum cost perfect matching
    pub fn matching(&mut self) -> Vec<usize> {
        if self.n == 0 {
            return Vec::new();
        }
        self.reduce_all();
        loop {
            self.find_zeros();
            if self.finished() {
                return self.select();
            }
            self.cover_zeros();
        }
    }

    fn reduce_all(&mut self) {
        // reduce each row by its minimum element
        for i in 0..self.n {
            let min_row = *self.matrix[(self.n * i)..(self.n * (i + 1))].iter().min().unwrap();
            for p in (self.n * i)..(self.n * (i + 1)) {
                self.matrix[p] -= min_row;
            }
        }
        // reduce each column by its minimum element
        for j in 0..self.n {
            let min_col = (j..(self.matrix.len())).step_by(self.n).map(|p| self.matrix[p]).min().unwrap();
            for p in (j..self.matrix.len()).step_by(self.n) {
                self.matrix[p] -= min_col;
            }
        }
    }

    fn find_zeros(&mut self) {
        self.zeros.clear();
        for (p, &v) in self.matrix.iter().enumerate() {
            if v == T::zero() {
                self.zeros.push(p);
            }
        }
    }

    fn finished(&mut self) -> bool {
        debug_assert_eq!(self.used_row.len(), self.n);
        debug_assert_eq!(self.used_col.len(), self.n);
        self.used_row.fill(false);
        self.used_col.fill(false);
        for &p in self.zeros.iter() {
            let i = p / self.n;
            let j = p % self.n;
            if !self.used_row[i] && !self.used_col[j] {
                self.used_row[i] = true;
                self.used_col[j] = true;
            }
        }
        self.used_row.len() == self.n
    }

    fn select(&mut self) -> Vec<usize> {
        debug_assert_eq!(self.used_row.len(), self.n);
        debug_assert_eq!(self.used_col.len(), self.n);
        self.used_row.fill(false);
        self.used_col.fill(false);
        let mut result = vec![0; self.n];
        for &p in self.zeros.iter() {
            let i = p / self.n;
            let j = p % self.n;
            if !self.used_row[i] && !self.used_col[j] {
                self.used_row[i] = true;
                self.used_col[j] = true;
                result[i] = j;
            }
        }
        result
    }

    fn cover_zeros(&mut self) {
        debug_assert_eq!(self.used_row.len(), self.n);
        debug_assert_eq!(self.used_col.len(), self.n);
        self.used_row.fill(false);
        self.used_col.fill(false);

        todo!("use rows and columns");
    }

    fn reduce_non_covered(&mut self) {
        let mut min_value = T::max_value();
        for i in 0..self.n {
            if self.used_row[i] {
                continue;
            }
            for j in 0..self.n {
                if self.used_col[j] {
                    continue;
                }
                min_value = min_value.min(self.matrix[self.n * i + j]);
            }
        }
        for i in 0..self.n {
            if self.used_row[i] {
                for j in 0..self.n {
                    if self.used_col[j] {
                        self.matrix[self.n * i + j] += min_value;
                    }
                }
            } else {
                for j in 0..self.n {
                    if !self.used_col[j] {
                        self.matrix[self.n * i + j] -= min_value;
                    }
                }
            }
        }
    }
}
