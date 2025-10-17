use rand::{rng, Rng, SeedableRng};

pub struct RollingHash {
    n: usize,
    hashes: Vec<i64>,
    powers: Vec<i64>,
}

impl RollingHash {
    fn get_mod() -> i64 {
        (1 << 61) - 1
    }

    pub fn generate_base() -> i64 {
        let mut master = rng();
        let mut rng = rand_pcg::Pcg64Mcg::from_rng(&mut master);
        rng.random_range(1..Self::get_mod())
    }

    pub fn from_vec<T: num_traits::NumCast>(s: &[T], base: i64) -> Self {
        let n = s.len();
        let mut hashes = vec![0; n + 1];
        let mut powers = vec![0; n + 1];
        powers[0] = 1;
        for i in 0..n {
            powers[i + 1] = Self::mul(powers[i], base);
            hashes[i + 1] = Self::add(Self::mul(hashes[i], base), s[i].to_i64().unwrap());
        }
        RollingHash {
            n,
            hashes,
            powers,
        }
    }

    pub fn prod(&self, l: usize, r: usize) -> i64 {
        debug_assert!(l <= r && r <= self.n);
        Self::add(self.hashes[r], Self::get_mod() - Self::mul(self.hashes[l], self.powers[r - l]))
    }

    pub fn concat(&self, hash1: i64, hash2: i64, len2: usize) {
        Self::add(Self::mul(hash1, self.powers[len2]), hash2);
    }

    fn add(a: i64, b: i64) -> i64 {
        let ret = a + b;
        if ret >= Self::get_mod() {
            ret - Self::get_mod()
        } else {
            ret
        }
    }

    fn mul(a: i64, b: i64) -> i64 {
        let ret = a as i128 * b as i128;
        Self::add((ret >> 61) as i64, (ret & Self::get_mod() as i128) as i64)
    }
}
