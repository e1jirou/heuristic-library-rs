pub fn divisors<T: num_traits::PrimInt>(n: T) -> Vec<T> {
    let mut ret = vec![];
    let mut d = T::one();
    while d * d < n {
        if n % d == T::zero() {
            ret.push(d);
            ret.push(n / d);
        }
        d = d + T::one();
    }
    if d * d == n {
        ret.push(d);
    }
    ret.sort();
    ret
}

pub fn prime_factorization<T: num_traits::PrimInt>(mut n: T) -> Vec<(T, usize)> {
    let mut ret = vec![];

    let mut push_if_divisible = |mut n: T, p: T| {
        if n % p == T::zero() {
            let mut cnt = 1;
            n = n / p;
            while n % p == T::zero() {
                cnt += 1;
                n = n / p;
            }
            ret.push((p, cnt));
        }
        n
    };
    let two = T::one() + T::one();
    let three = two + T::one();
    let four = three + T::one();
    let five = four + T::one();

    n = push_if_divisible(n, two);
    n = push_if_divisible(n, three);

    let mut p = five;
    while p * p <= n {
        n = push_if_divisible(n, p);
        p = p + two;
        n = push_if_divisible(n, p);
        p = p + four;
    }
    if n >= two {
        ret.push((n, 1));
    }
    ret
}

pub struct Eratosthenes {
    n: usize,
    sieve: Vec<usize>,
    pub prime_numbers: Vec<usize>,
}

impl Eratosthenes {
    pub fn new(n: usize) -> Self {
        let mut sieve = vec![0; n];
        let mut prime_numbers = vec![];
        for p in 2..n {
            if sieve[p] > 0 {
                continue;
            }
            prime_numbers.push(p);
            for i in (p..n).step_by(p) {
                sieve[i] = p;
            }
        }
        Self {
            n,
            sieve,
            prime_numbers,
        }
    }

    pub fn prime_factorization(&self, mut x: usize) -> Vec<usize> {
        debug_assert!(1 <= x && x < self.n);
        let mut ret = vec![];
        while x >= 2 {
            let p = self.sieve[x];
            ret.push(p);
            x /= p;
        }
        ret
    }
}
