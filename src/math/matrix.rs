pub trait Ring:
    num_traits::Zero +
    num_traits::One +
    std::ops::Add<Output = Self> +
    std::ops::AddAssign +
    std::ops::Sub<Output = Self> +
    std::ops::SubAssign +
    std::ops::Mul<Output = Self> +
    std::ops::MulAssign +
    Copy
{}

#[derive(Debug, Clone)]
pub struct Matrix<T: Ring> {
    n: usize,
    m: usize,
    a: Vec<T>,
}

impl<T: Ring> Matrix<T> {
    pub fn new(n: usize, m: usize) -> Self {
        let a = vec![T::zero(); n * m];
        Matrix { n, m, a }
    }

    pub fn id(n: usize) -> Self {
        let mut ret = Matrix::new(n, n);
        for i in 0..n {
            ret.set(i, i, T::one());
        }
        ret
    }

    pub fn get(&self, i: usize, j: usize) -> T {
        debug_assert!(i < self.n && j < self.m);
        self.a[i * self.m + j]
    }

    pub fn set(&mut self, i: usize, j: usize, v: T) {
        debug_assert!(i < self.n && j < self.m);
        self.a[i * self.m + j] = v;
    }

    pub fn pow(&self, mut exp: u64) -> Self {
        debug_assert!(self.n == self.m);
        let mut res = Matrix::id(self.n);
        let mut base = self.clone();
        while exp > 0 {
            if exp & 1 == 1 {
                res *= &base;
            }
            base = &base * &base;
            exp >>= 1;
        }
        res
    }
}

impl<T: Ring> std::ops::AddAssign<&Matrix<T>> for Matrix<T> {
    fn add_assign(&mut self, other: &Matrix<T>) {
        debug_assert!(self.n == other.n && self.m == other.m);
        for i in 0..self.n {
            for j in 0..self.m {
                let v = self.get(i, j) + other.get(i, j);
                self.set(i, j, v);
            }
        }
    }
}

impl<T: Ring> std::ops::AddAssign for Matrix<T> {
    fn add_assign(&mut self, other: Self) {
        self.add_assign(&other);
    }
}

impl<T: Ring> std::ops::SubAssign<&Matrix<T>> for Matrix<T> {
    fn sub_assign(&mut self, other: &Matrix<T>) {
        debug_assert!(self.n == other.n && self.m == other.m);
        for i in 0..self.n {
            for j in 0..self.m {
                let v = self.get(i, j) - other.get(i, j);
                self.set(i, j, v);
            }
        }
    }
}

impl<T: Ring> std::ops::SubAssign for Matrix<T> {
    fn sub_assign(&mut self, other: Self) {
        self.sub_assign(&other);
    }
}

impl<T: Ring> std::ops::MulAssign<&Matrix<T>> for Matrix<T> {
    fn mul_assign(&mut self, other: &Matrix<T>) {
        debug_assert!(self.m == other.n);
        let mut res = Matrix::new(self.n, other.m);
        for i in 0..self.n {
            for j in 0..other.m {
                let mut v = T::zero();
                for k in 0..self.m {
                    v += self.get(i, k) * other.get(k, j);
                }
                res.set(i, j, v);
            }
        }
        *self = res;
    }
}

impl<T: Ring> std::ops::MulAssign for Matrix<T> {
    fn mul_assign(&mut self, other: Self) {
        self.mul_assign(&other);
    }
}

impl<T: Ring> std::ops::Add for Matrix<T> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl<T: Ring> std::ops::Add<&Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn add(mut self, other: &Matrix<T>) -> Matrix<T> {
        self += other;
        self
    }
}

impl<T: Ring> std::ops::Add<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, mut other: Matrix<T>) -> Matrix<T> {
        other += self;
        other
    }
}

impl<T: Ring> std::ops::Add for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, other: &Matrix<T>) -> Matrix<T> {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl<T: Ring> std::ops::Sub for Matrix<T> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}

impl<T: Ring> std::ops::Sub<&Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn sub(mut self, other: &Matrix<T>) -> Matrix<T> {
        self -= other;
        self
    }
}

impl<T: Ring> std::ops::Sub<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, other: Matrix<T>) -> Matrix<T> {
        let mut result = self.clone();
        result -= &other;
        result
    }
}

impl<T: Ring> std::ops::Sub for &Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, other: &Matrix<T>) -> Matrix<T> {
        let mut result = self.clone();
        result -= other;
        result
    }
}

impl<T: Ring> std::ops::Mul for Matrix<T> {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self *= other;
        self
    }
}

impl<T: Ring> std::ops::Mul<&Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(mut self, other: &Matrix<T>) -> Matrix<T> {
        self *= other;
        self
    }
}

impl<T: Ring> std::ops::Mul<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, other: Matrix<T>) -> Matrix<T> {
        debug_assert!(self.m == other.n);
        let mut res = Matrix::new(self.n, other.m);
        for i in 0..self.n {
            for j in 0..other.m {
                let mut v = T::zero();
                for k in 0..self.m {
                    v += self.get(i, k) * other.get(k, j);
                }
                res.set(i, j, v);
            }
        }
        res
    }
}

impl<T: Ring> std::ops::Mul for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, other: &Matrix<T>) -> Matrix<T> {
        debug_assert!(self.m == other.n);
        let mut res = Matrix::new(self.n, other.m);
        for i in 0..self.n {
            for j in 0..other.m {
                let mut v = T::zero();
                for k in 0..self.m {
                    v += self.get(i, k) * other.get(k, j);
                }
                res.set(i, j, v);
            }
        }
        res
    }
}
