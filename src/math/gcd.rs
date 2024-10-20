pub fn gcd<T: num_traits::PrimInt>(a: T, b: T) -> T {
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<T: num_traits::PrimInt>(a: T, b: T) -> T {
    debug_assert!(a != T::zero() && b != T::zero());
    let g = gcd(a, b);
    (a / g) * b
}

// a * x + b * y = gcd(a, b)
// return (gcd(a, b), x, y)
pub fn extended_euclid<T: num_traits::PrimInt>(a: T, b: T) -> (T, T, T) {
    if b > T::zero() {
        let (g, x, y) = extended_euclid(b, a % b);
        (g, y, x - (a / b) * y)
    } else {
        (a, T::one(), T::zero())
    }
}
