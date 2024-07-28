pub fn log_sum_exp<T: num_traits::Float + std::iter::Sum>(v: &[T]) -> T {
    v.iter().map(|&x| x.exp()).sum::<T>().ln()
}

pub fn log_sum_pow<T: num_traits::Float + std::iter::Sum>(base: T, v: &[T]) -> T {
    let max_v = *v.iter().max_by(|&&x, &y| x.partial_cmp(y).unwrap()).unwrap();
    v.iter().map(|&x| base.powf(x - max_v)).sum::<T>().log(base) + max_v
}

pub fn euclid_norm<T: num_traits::Float + std::iter::Sum>(v: &[T]) -> T {
    v.iter().map(|&x| x * x).sum::<T>().sqrt()
}

pub fn p_norm<T: num_traits::Float + std::iter::Sum>(p: T, v: &[T]) -> T {
    let max_v = *v.iter().max_by(|&&x, &y| x.partial_cmp(y).unwrap()).unwrap();
    v.iter().map(|&x| (x / max_v).powf(p)).sum::<T>().powf(T::one() / p) * max_v
}
