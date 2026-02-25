pub fn bit_set<T: num_traits::PrimInt + std::ops::BitOrAssign>(x: &mut T, p: usize) -> bool {
    let y = T::one() << p;
    if (*x & y) == T::zero() {
        *x |= y;
        true
    } else {
        false
    }
}

pub fn bit_get<T: num_traits::PrimInt>(x: T, p: usize) -> bool {
    (x & (T::one() << p)) != T::zero()
}

pub fn bit_width<T: num_traits::PrimInt>(x: T) -> usize {
    8 * std::mem::size_of::<T>() - x.leading_zeros() as usize
}

#[inline(always)]
pub fn pop_one<T: num_traits::PrimInt + std::ops::BitXorAssign>(x: &mut T) -> Option<u32> {
    if *x == T::zero() {
        return None;
    }
    let ret = x.trailing_zeros();
    *x ^= T::one() << ret as usize;
    Some(ret)
}

#[inline(always)]
pub fn select128(b: u128, i: usize) -> usize {
    debug_assert!(i < b.count_ones() as usize);
    let low64 = b as u64;
    let low64_cnt = low64.count_ones() as usize;
    if i < low64_cnt {
        select64(low64, i)
    } else {
        let high64 = (b >> 64) as u64;
        select64(high64, i - low64_cnt) + 64
    }
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn select64(b: u64, i: usize) -> usize {
    debug_assert!(i < b.count_ones() as usize);
    unsafe { std::arch::x86_64::_pdep_u64(1 << i, b) }.trailing_zeros() as usize
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
pub fn select64(b: u64, i: usize) -> usize {
    debug_assert!(i < b.count_ones() as usize);
    let low32 = b as u32;
    let low32_cnt = low32.count_ones() as usize;
    if i < low32_cnt {
        select32(low32, i)
    } else {
        let high32 = (b >> 32) as u32;
        select32(high32, i - low32_cnt) + 32
    }
}

#[inline(always)]
pub fn select32(mut b: u32, i: usize) -> usize {
    debug_assert!(i < b.count_ones() as usize);
    for _ in 0..i {
        b &= b - 1;
    }
    b.trailing_zeros() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select32() {
        // 0b00010010_00000001_00000000_00000001 = 0x12010001
        let b: u32 = 0b00010010_00000001_00000000_00000001;
        let ones: Vec<usize> = (0..32).filter(|&k| (b & (1 << k)) != 0).collect();
        for (i, &pos) in ones.iter().enumerate() {
            assert_eq!(select32(b, i), pos);
        }

        // 全ビット立っている場合
        let b: u32 = !0;
        for i in 0..32 {
            assert_eq!(select32(b, i), i);
        }

        // 1ビットのみ
        for k in 0..32 {
            let b = 1u32 << k;
            assert_eq!(select32(b, 0), k);
        }
    }

    #[test]
    fn test_select64() {
        // On x86_64, select64 uses the BMI2 `pdep` instruction.
        // If the CPU doesn't support BMI2, executing it can trap.
        #[cfg(target_arch = "x86_64")]
        {
            if !std::is_x86_feature_detected!("bmi2") {
                return;
            }
        }

        // Specific bit pattern
        // 0b0001_0010_0000_0001_0000_0000_0000_0001 (low 32)
        // and some bits in the high 32.
        let b: u64 = 0x0000_0000_1201_0001u64 | (1u64 << 40) | (1u64 << 63) | (1u64 << 33);
        let ones: Vec<usize> = (0..64).filter(|&k| (b & (1u64 << k)) != 0).collect();
        for (i, &pos) in ones.iter().enumerate() {
            assert_eq!(select64(b, i), pos);
        }

        // All bits set
        let b: u64 = !0;
        for i in 0..64 {
            assert_eq!(select64(b, i), i);
        }

        // Single bit set
        for k in 0..64 {
            let b = 1u64 << k;
            assert_eq!(select64(b, 0), k);
        }
    }
}