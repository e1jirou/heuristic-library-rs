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

pub fn pop_one<T: num_traits::PrimInt + std::ops::BitXorAssign>(x: &mut T) -> Option<u32> {
    if *x == T::zero() {
        return None;
    }
    let ret = x.trailing_zeros();
    *x ^= T::one() << ret as usize;
    Some(ret)
}
