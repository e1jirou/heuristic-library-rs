use super::convolution::convolution;
use super::mod_int::ModInt;
use num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub struct FormalPowerSeries<const MOD: u32> {
    pub f: Vec<ModInt<MOD>>,
}

impl<const MOD: u32> FormalPowerSeries<MOD> {
    pub fn new(f: Vec<ModInt<MOD>>) -> Self {
        FormalPowerSeries { f }
    }

    pub fn truncate(&mut self, deg: usize) {
        if self.f.len() > deg {
            self.f.truncate(deg + 1);
        }
    }

    pub fn pow(&self, mut exp: u64, deg: usize) -> Self {
        let mut res = FormalPowerSeries::new(vec![ModInt::one(); 1]);
        let mut base = self.clone();
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base.clone();
                res.truncate(deg);
            }
            base *= base.clone();
            base.truncate(deg);
            exp >>= 1;
        }
        res
    }

    pub fn inv(&self, deg: usize) -> Self {
        debug_assert!(!self.f[0].is_zero());
        let n = deg + 1;
        let mut g = Self::new(vec![self.f[0].inv()]);
        let mut len = 1;
        while len < n {
            len <<= 1;
            let mut h = Self::new(self.f[..len.min(self.f.len())].to_vec()) * g.clone();
            for i in 0..h.f.len() {
                h.f[i] = -h.f[i];
            }
            h.f[0] += ModInt::raw(2);
            g *= h;
            g.truncate(len + 1);
        }
        g.truncate(deg);
        g
    }
}

impl<const MOD: u32> std::ops::AddAssign for FormalPowerSeries<MOD> {
    fn add_assign(&mut self, other: Self) {
        let n = self.f.len().max(other.f.len());
        self.f.resize(n, ModInt::zero());
        for i in 0..other.f.len() {
            self.f[i] += other.f[i];
        }
    }
}

impl<const MOD: u32> std::ops::SubAssign for FormalPowerSeries<MOD> {
    fn sub_assign(&mut self, other: Self) {
        let n = self.f.len().max(other.f.len());
        self.f.resize(n, ModInt::zero());
        for i in 0..other.f.len() {
            self.f[i] -= other.f[i];
        }
    }
}

impl<const MOD: u32> std::ops::MulAssign for FormalPowerSeries<MOD> {
    fn mul_assign(&mut self, other: Self) {
        let f = convolution(self.f.clone(), other.f.clone());
        self.f = f;
    }
}

impl<const MOD: u32> std::ops::Add for FormalPowerSeries<MOD> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut res = self;
        res += other;
        res
    }
}

impl<const MOD: u32> std::ops::Sub for FormalPowerSeries<MOD> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut res = self;
        res -= other;
        res
    }
}

impl<const MOD: u32> std::ops::Mul for FormalPowerSeries<MOD> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut res = self;
        res *= other;
        res
    }
}

pub fn total_product<const MOD: u32>(
    fps_list: Vec<FormalPowerSeries<MOD>>,
    deg: usize,
) -> FormalPowerSeries<MOD> {
    use std::collections::BinaryHeap;

    struct Entry<const MOD: u32> {
        size: usize,
        fps: FormalPowerSeries<MOD>,
    }
    impl<const MOD: u32> PartialEq for Entry<MOD> {
        fn eq(&self, other: &Self) -> bool {
            self.size == other.size
        }
    }
    impl<const MOD: u32> Eq for Entry<MOD> {}
    impl<const MOD: u32> PartialOrd for Entry<MOD> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.size.cmp(&other.size).reverse())
        }
    }
    impl<const MOD: u32> Ord for Entry<MOD> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.size.cmp(&other.size).reverse()
        }
    }
    let mut heap = BinaryHeap::new();
    for fps in fps_list {
        heap.push(Entry {
            size: fps.f.len(),
            fps,
        });
    }
    while heap.len() > 1 {
        let e1 = heap.pop().unwrap();
        let e2 = heap.pop().unwrap();
        let mut merged = e1.fps * e2.fps;
        merged.truncate(deg);
        heap.push(Entry {
            size: merged.f.len(),
            fps: merged,
        });
    }
    heap.pop().unwrap().fps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fps_inv_simple() {
        const MOD: u32 = 998244353;
        // Test f(x) = 1, should give inv = 1
        let f = FormalPowerSeries::<MOD>::new(vec![ModInt::from_u64(1)]);
        let inv = f.inv(5);
        assert_eq!(inv.f[0], ModInt::from_u64(1));
        for i in 1..inv.f.len() {
            assert_eq!(inv.f[i], ModInt::zero());
        }
    }

    #[test]
    fn test_fps_inv_product() {
        const MOD: u32 = 998244353;
        // Test f(x) = 1 + x, verify f * inv = 1 (mod x^deg)
        let f = FormalPowerSeries::<MOD>::new(vec![ModInt::from_u64(1), ModInt::from_u64(1)]);
        let inv = f.inv(5);

        // Verify f * inv = 1 (mod x^5)
        let mut product = f * inv;
        product.truncate(4); // deg - 1

        assert_eq!(product.f[0], ModInt::from_u64(1));
        for i in 1..product.f.len() {
            assert_eq!(product.f[i], ModInt::zero());
        }
    }

    #[test]
    fn test_fps_inv_alternating_series() {
        const MOD: u32 = 998244353;
        // Test f(x) = 1 + x, the inverse should be 1 - x + x^2 - x^3 + x^4 - ...
        let f = FormalPowerSeries::<MOD>::new(vec![ModInt::from_u64(1), ModInt::from_u64(1)]);
        let inv = f.inv(6);

        // Check coefficients of the geometric series
        assert_eq!(inv.f[0], ModInt::from_u64(1));
        assert_eq!(inv.f[1], ModInt::from_u64(MOD as u64 - 1)); // -1 mod MOD
        assert_eq!(inv.f[2], ModInt::from_u64(1));
        assert_eq!(inv.f[3], ModInt::from_u64(MOD as u64 - 1)); // -1 mod MOD
        assert_eq!(inv.f[4], ModInt::from_u64(1));
        assert_eq!(inv.f[5], ModInt::from_u64(MOD as u64 - 1)); // -1 mod MOD
    }

    #[test]
    fn test_fps_inv_complex() {
        const MOD: u32 = 998244353;
        // Test f(x) = 2 + 3x + x^2
        let f = FormalPowerSeries::<MOD>::new(vec![
            ModInt::from_u64(2),
            ModInt::from_u64(3),
            ModInt::from_u64(1),
        ]);
        let inv = f.inv(5);

        // Verify f * inv = 1 (mod x^5)
        let mut product = f * inv;
        product.truncate(4); // deg - 1

        assert_eq!(product.f[0], ModInt::from_u64(1));
        for i in 1..product.f.len() {
            assert_eq!(product.f[i], ModInt::zero());
        }
    }
}
