// floor(a^{1 / k})
pub fn kth_root(a: u64, k: u64) -> u64 {
    debug_assert!(k >= 1);
    if a <= 1 || k == 1 {
        return a;
    }
    if k >= 64 {
        return 1;
    }
    // return base.pow(exp) <= a
    let check = |base: u64, mut exp: u64, a: u64| {
        let mut x: u64 = 1;
        let mut m = base;
        loop {
            if (exp & 1) == 1 {
                x = match x.checked_mul(m) {
                    Some(n) => n,
                    None => {
                        return false;
                    }
                }
            }
            exp >>= 1;
            if exp == 0 {
                break;
            }
            m = match m.checked_mul(m) {
                Some(n) => n,
                None => {
                    return false;
                }
            }
        }
        return x <= a;
    };
    // binary search
    let mut lower = 0;
    let mut upper = a - 1;
    while lower < upper {
        let center = (lower + upper + 1) / 2;
        if check(center, k, a) {
            lower = center;
        } else {
            upper = center - 1;
        }
    }
    lower
}
