#[derive(Debug, Clone, Copy)]
pub struct XorShift32 {
    state: u32,
}

impl XorShift32 {
    pub fn new(mut seed: u32) -> Self {
        if seed == 0 {
            seed = u32::MAX;
        }
        Self { state: seed }
    }

    /// 1..=u32::MAX
    #[inline(always)]
    pub fn gen(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    /// 0.0..=1.0
    #[inline(always)]
    pub fn gen_f32(&mut self) -> f32 {
        self.gen() as f32 / ((1u64 << 32) as f32)
    }

    /// 0.0..=1.0
    #[inline(always)]
    pub fn gen_f64(&mut self) -> f64 {
        self.gen() as f64 / ((1u64 << 32) as f64)
    }

    /// l..r
    #[inline(always)]
    pub fn gen_range(&mut self, l: usize, r: usize) -> usize {
        debug_assert!(l < r);
        debug_assert!(r as u64 <= 1 << 32);
        l + (((r - l) as u64 * self.gen() as u64) >> 32) as usize
    }

    /// l..=r
    #[inline(always)]
    pub fn gen_range_f32(&mut self, l: f32, r: f32) -> f32 {
        debug_assert!(l <= r);
        l + self.gen() as f32 * ((r - l) / ((1u64 << 32) as f32))
    }

    /// l..=r
    #[inline(always)]
    pub fn gen_range_f64(&mut self, l: f64, r: f64) -> f64 {
        debug_assert!(l <= r);
        l + self.gen() as f64 * ((r - l) / ((1u64 << 32) as f64))
    }
}
