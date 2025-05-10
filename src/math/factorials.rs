use crate::math::mod_int::ModInt;

pub struct Factorials<const MOD: u32> {
    n: usize,
    fct: Vec<ModInt<MOD>>,
    inv_fct: Vec<ModInt<MOD>>,
}

impl<const MOD: u32> Factorials<MOD> {
    pub fn new(n: usize) -> Self {
        let mut fct = vec![ModInt::<MOD>::zero(); n + 1];
        let mut inv_fct = vec![ModInt::<MOD>::zero(); n + 1];
        fct[0] = ModInt::<MOD>::one();
        for i in 1..=n {
            fct[i] = fct[i - 1] * ModInt::<MOD>::raw(i as u32);
        }
        inv_fct[n] = fct[n].inv();
        for i in (1..=n).rev() {
            inv_fct[i - 1] = inv_fct[i] * ModInt::<MOD>::raw(i as u32);
        }
        Self {
            n,
            fct,
            inv_fct,
        }
    }

    pub fn fct(&self, n: usize) -> ModInt<MOD> {
        assert!(n <= self.n);
        self.fct[n]
    }

    pub fn inv_fct(&self, n: usize) -> ModInt<MOD> {
        assert!(n <= self.n);
        self.inv_fct[n]
    }

    pub fn comb(&self, n: usize, k: usize) -> ModInt<MOD> {
        assert!(n <= self.n);
        if k > n {
            return ModInt::<MOD>::zero();
        }
        self.fct[n] * self.inv_fct[k] * self.inv_fct[n - k]
    }

    pub fn perm(&self, n: usize, k: usize) -> ModInt<MOD> {
        assert!(k <= n && n <= self.n);
        self.fct[n] * self.inv_fct[n - k]
    }

    pub fn inv(&self, n: usize) -> ModInt<MOD> {
        assert!(1 <= n && n <= self.n);
        self.fct[n - 1] * self.inv_fct[n]
    }
}
