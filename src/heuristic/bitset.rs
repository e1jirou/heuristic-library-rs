#[macro_export]
macro_rules! make_bitset {
    ($n:literal) => {
        StaticBitSet<$n, { ($n + 63) / 64 }>
    };
}

#[derive(Debug, Clone)]
pub struct StaticBitSet<const N: usize, const ARRAY_SIZE: usize> {
    data: [u64; ARRAY_SIZE],
}

impl<const N: usize, const ARRAY_SIZE: usize> StaticBitSet<N, ARRAY_SIZE> {
    pub const fn new() -> Self {
        debug_assert!(N > 0);
        debug_assert!(ARRAY_SIZE == (N + 63) / 64);
        Self {
            data: [0; ARRAY_SIZE],
        }
    }

    pub fn get(&self, i: usize) -> bool {
        debug_assert!(i < N);
        (self.data[i / 64] & (1 << (i & 63))) != 0
    }

    pub fn set(&mut self, i: usize, flag: bool) {
        debug_assert!(i < N);
        if flag {
            self.data[i / 64] |= 1 << (i & 63);
        } else {
            self.data[i / 64] &= !(1 << (i & 63));
        }
    }

    pub fn any(&self) -> bool {
        self.data.iter().any(|&x| x != 0)
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    pub fn count(&self) -> u32 {
        self.data.iter().map(|&x| x.count_ones()).sum()
    }

    pub fn range_set(&mut self, l: usize, r: usize, flag: bool) {
        debug_assert!(l <= r && r <= N);
        if flag {
            if l / 64 == r / 64 {
                self.data[l / 64] |= (1u64 << (r & 63)).wrapping_sub(1 << (l & 63));
                return;
            }
            self.data[l / 64] |= (0 as u64).wrapping_sub(1 << (l & 63));
            for i in (l / 64 + 1)..(r / 64) {
                self.data[i] = !0;
            }
            if r & 63 != 0 {
                self.data[r / 64] |= (1 << (r & 63)) - 1;
            }
        } else {
            if l / 64 == r / 64 {
                self.data[l / 64] &= !((1u64 << (r & 63)).wrapping_sub(1 << (l & 63)));
                return;
            }
            self.data[l / 64] &= !((0 as u64).wrapping_sub(1 << (l & 63)));
            for i in (l / 64 + 1)..(r / 64) {
                self.data[i] = 0;
            }
            if r & 63 != 0 {
                self.data[r / 64] &= !((1 << (r & 63)) - 1);
            }
        }
    }

    pub fn get_ones(&self) -> Vec<usize> {
        let mut ones = vec![];
        for i in 0..self.data.len() {
            let mut x = self.data[i];
            while x > 0 {
                let p = x.trailing_zeros() as usize;
                x ^= 1 << p;
                ones.push(64 * i + p);
            }
        }
        ones
    }
}

impl<const N: usize, const ARRAY_SIZE: usize> std::ops::BitAndAssign
    for StaticBitSet<N, ARRAY_SIZE>
{
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..ARRAY_SIZE {
            self.data[i] &= rhs.data[i];
        }
    }
}

impl<const N: usize, const ARRAY_SIZE: usize> std::ops::BitOrAssign
    for StaticBitSet<N, ARRAY_SIZE>
{
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..ARRAY_SIZE {
            self.data[i] |= rhs.data[i];
        }
    }
}

impl<const N: usize, const ARRAY_SIZE: usize> std::ops::BitXorAssign
    for StaticBitSet<N, ARRAY_SIZE>
{
    fn bitxor_assign(&mut self, rhs: Self) {
        for i in 0..ARRAY_SIZE {
            self.data[i] ^= rhs.data[i];
        }
    }
}

impl<const N: usize, const ARRAY_SIZE: usize> std::ops::BitAnd for StaticBitSet<N, ARRAY_SIZE> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res &= rhs;
        res
    }
}

impl<const N: usize, const ARRAY_SIZE: usize> std::ops::BitOr for StaticBitSet<N, ARRAY_SIZE> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res |= rhs;
        res
    }
}

impl<const N: usize, const ARRAY_SIZE: usize> std::ops::BitXor for StaticBitSet<N, ARRAY_SIZE> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res ^= rhs;
        res
    }
}

impl<const N: usize, const ARRAY_SIZE: usize> std::ops::Not for StaticBitSet<N, ARRAY_SIZE> {
    type Output = Self;
    fn not(self) -> Self::Output {
        let mut res = self.clone();
        for i in 0..ARRAY_SIZE {
            res.data[i] = !res.data[i];
        }
        if (N & 63) != 0 {
            res.data[ARRAY_SIZE - 1] &= (1u64 << (N & 63)) - 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 型エイリアス
    type BitSet64 = StaticBitSet<64, 1>;
    type BitSet256 = StaticBitSet<256, 4>;

    #[test]
    fn test_basic_operations() {
        let mut bs: BitSet64 = StaticBitSet::new();

        // 基本的なset/getテスト
        assert!(!bs.get(0));
        bs.set(0, true);
        assert!(bs.get(0));
        bs.set(0, false);
        assert!(!bs.get(0));

        // 複数ビットのテスト
        bs.set(5, true);
        bs.set(10, true);
        bs.set(63, true);

        assert!(bs.get(5));
        assert!(bs.get(10));
        assert!(bs.get(63));
        assert!(!bs.get(1));

        assert!(bs.any());
        assert_eq!(bs.count(), 3);
    }

    #[test]
    fn test_clear() {
        let mut bs: BitSet64 = StaticBitSet::new();
        bs.set(1, true);
        bs.set(10, true);
        assert!(bs.any());

        bs.clear();
        assert!(!bs.any());
        assert_eq!(bs.count(), 0);
    }

    #[test]
    fn test_get_ones() {
        let mut bs: BitSet64 = StaticBitSet::new();
        bs.set(0, true);
        bs.set(5, true);
        bs.set(63, true);

        let ones = bs.get_ones();
        assert_eq!(ones, vec![0, 5, 63]);
    }

    #[test]
    fn test_bitwise_operations() {
        let mut bs1: BitSet64 = StaticBitSet::new();
        let mut bs2: BitSet64 = StaticBitSet::new();

        bs1.set(0, true);
        bs1.set(5, true);

        bs2.set(5, true);
        bs2.set(10, true);

        // AND
        let and_result = bs1.clone() & bs2.clone();
        assert!(and_result.get(5));
        assert!(!and_result.get(0));
        assert!(!and_result.get(10));

        // OR
        let or_result = bs1.clone() | bs2.clone();
        assert!(or_result.get(0));
        assert!(or_result.get(5));
        assert!(or_result.get(10));

        // XOR
        let xor_result = bs1.clone() ^ bs2.clone();
        assert!(xor_result.get(0));
        assert!(!xor_result.get(5));
        assert!(xor_result.get(10));
    }

    #[test]
    fn test_not_operation() {
        let mut bs: BitSet64 = StaticBitSet::new();
        bs.set(0, true);
        bs.set(63, true);

        let not_result = !bs;
        assert!(!not_result.get(0));
        assert!(!not_result.get(63));
        assert!(not_result.get(1));
        assert!(not_result.get(32));
        assert!(not_result.get(62));
    }

    #[test]
    fn test_range_set() {
        let mut bs: BitSet64 = StaticBitSet::new();

        // 範囲設定のテスト
        bs.range_set(5, 10, true);
        for i in 5..10 {
            assert!(bs.get(i), "Bit {} should be set", i);
        }
        assert!(!bs.get(4));
        assert!(!bs.get(10));

        // 範囲クリアのテスト
        bs.range_set(6, 8, false);
        assert!(bs.get(5));
        assert!(!bs.get(6));
        assert!(!bs.get(7));
        assert!(bs.get(8));
        assert!(bs.get(9));
    }

    #[test]
    fn test_large_bitset() {
        let mut bs: BitSet256 = StaticBitSet::new();

        bs.set(0, true);
        bs.set(100, true);
        bs.set(200, true);
        bs.set(255, true);

        assert_eq!(bs.count(), 4);
        let ones = bs.get_ones();
        assert_eq!(ones, vec![0, 100, 200, 255]);
    }

    #[test]
    fn test_macro() {
        let mut bs: make_bitset!(100) = StaticBitSet::new();
        bs.set(99, true);
        assert!(bs.get(99));
        assert_eq!(bs.count(), 1);
    }

    #[test]
    fn test_edge_cases() {
        let mut bs: BitSet64 = StaticBitSet::new();

        // 境界値のテスト
        bs.set(0, true);
        bs.set(63, true);
        assert!(bs.get(0));
        assert!(bs.get(63));

        // 範囲設定のテスト（0から64まで全部）
        bs.clear();
        bs.range_set(0, 64, true);
        for i in 0..64 {
            assert!(bs.get(i));
        }
        assert_eq!(bs.count(), 64);
    }
}
