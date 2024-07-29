pub fn log_sum_exp<T: num_traits::Float + std::iter::Sum>(v: &[T]) -> T {
    debug_assert!(!v.is_empty());
    v.iter().map(|&x| x.exp()).sum::<T>().ln()
}

pub fn log_sum_pow<T: num_traits::Float + std::iter::Sum>(base: T, v: &[T]) -> T {
    debug_assert!(!v.is_empty());
    let max_v = *v.iter().max_by(|&&x, &y| x.partial_cmp(y).unwrap()).unwrap();
    v.iter().map(|&x| base.powf(x - max_v)).sum::<T>().log(base) + max_v
}

pub fn euclid_norm<T: num_traits::Float + std::iter::Sum>(v: &[T]) -> T {
    debug_assert!(!v.is_empty());
    v.iter().map(|&x| x * x).sum::<T>().sqrt()
}

pub fn p_norm<T: num_traits::Float + std::iter::Sum>(p: T, v: &[T]) -> T {
    debug_assert!(!v.is_empty());
    let max_v = *v.iter().max_by(|&&x, &y| x.partial_cmp(y).unwrap()).unwrap();
    v.iter().map(|&x| (x / max_v).powf(p)).sum::<T>().powf(T::one() / p) * max_v
}

// if eps = 0.0 then smu(a, b) = max(a, b)
pub fn smooth_maximum_unit<T: num_traits::Float>(a: T, b: T, eps: T) -> T {
    let d = a - b;
    (a + b + (d * d + eps).sqrt()) / (T::one() + T::one())
}