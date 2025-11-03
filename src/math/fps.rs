use super::convolution::convolution;
use super::mod_int::ModInt;
use num_traits::{Zero, One};

#[derive(Debug, Clone)]
pub struct FormalPowerSeries<const MOD: u32> {
    pub f: Vec<ModInt<MOD>>,
}

impl<const MOD: u32> FormalPowerSeries<MOD> {
    pub fn new(f: Vec<ModInt<MOD>>) -> Self {
        FormalPowerSeries { f }
    }

    pub fn pow(&self, mut exp: u64, deg: usize) -> Self {
        let mut res = FormalPowerSeries::new(vec![ModInt::one(); 1]);
        let mut base = self.clone();
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base.clone();
                if res.f.len() > deg {
                    res.f.truncate(deg + 1);
                }
            }
            base *= base.clone();
            if base.f.len() > deg {
                base.f.truncate(deg + 1);
            }
            exp >>= 1;
        }
        res
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
    impl <const MOD: u32> PartialEq for Entry<MOD> {
        fn eq(&self, other: &Self) -> bool {
            self.size == other.size
        }
    }
    impl <const MOD: u32> Eq for Entry<MOD> {}
    impl <const MOD: u32> PartialOrd for Entry<MOD> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.size.cmp(&other.size).reverse())
        }
    }
    impl <const MOD: u32> Ord for Entry<MOD> {
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
        if merged.f.len() > deg {
            merged.f.truncate(deg + 1);
        }
        heap.push(Entry {
            size: merged.f.len(),
            fps: merged,
        });
    }
    heap.pop().unwrap().fps
}
