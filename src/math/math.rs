use super::mod_int::{inv_gcd, safe_mod};

pub fn pow_mod(x: i64, mut n: i64, m: i32) -> i64 {
    debug_assert!(n >= 0);
    debug_assert!(m >= 1);
    if m == 1 {
        return 0;
    }
    let mut r = 1;
    let mut y = safe_mod(x, m as i64);
    while n != 0 {
        if (n & 1) == 1 {
            r = r * y % m as i64;
        }
        y = y * y % m as i64;
        n >>= 1;
    }
    r
}

// (rem, mod)
pub fn crt(r: &[i64], m: &[i64]) -> (i64, i64) {
    debug_assert!(r.len() == m.len());
    let n = r.len();
    // Contracts: 0 <= r0 < m0
    let mut r0 = 0;
    let mut m0 = 1;
    for i in 0..n {
        debug_assert!(m[i] >= 1);
        let mut r1 = safe_mod(r[i], m[i]);
        let mut m1 = m[i];
        if m0 < m1 {
            std::mem::swap(&mut r0, &mut r1);
            std::mem::swap(&mut m0, &mut m1);
        }
        if m0 % m1 == 0 {
            if r0 % m1 != r1 {
                return (0, 0);
            }
            continue;
        }
        // assume: m0 > m1, lcm(m0, m1) >= 2 * max(m0, m1)

        // (r0, m0), (r1, m1) -> (r2, m2 = lcm(m0, m1));
        // r2 % m0 = r0
        // r2 % m1 = r1
        // -> (r0 + x*m0) % m1 = r1
        // -> x*u0*g = r1-r0 (mod u1*g) (u0*g = m0, u1*g = m1)
        // -> x = (r1 - r0) / g * inv(u0) (mod u1)

        // im = inv(u0) (mod u1) (0 <= im < u1)
        let (g, im) = inv_gcd(m0, m1);

        let u1 = m1 / g;
        // |r1 - r0| < (m0 + m1) <= lcm(m0, m1)
        if (r1 - r0) % g != 0 {
            return (0, 0);
        }
        // u1 * u1 <= m1 * m1 / g / g <= m0 * m1 / g = lcm(m0, m1)
        let x = (r1 - r0) / g % u1 * im % u1;

        // |r0| + |m0 * x|
        // < m0 + m0 * (u1 - 1)
        // = m0 + m0 * m1 / g - m0
        // = lcm(m0, m1)
        r0 += x * m0;
        m0 *= u1;  // -> lcm(m0, m1)
        if r0 < 0 {
            r0 += m0;
        }
    }
    (r0, m0)
}

// return sum_{i=0}^{n-1} floor((ai + b) / m) (mod 2^64)
fn floor_sum_unsigned(mut n: u64, mut m: u64, mut a: u64, mut b: u64) -> u64 {
    let mut ans = 0;
    loop {
        if a >= m {
            ans += n * (n - 1) / 2 * (a / m);
            a %= m;
        }
        if b >= m {
            ans += n * (b / m);
            b %= m;
        }

        let y_max = a * n + b;
        if y_max < m {
            break;
        }
        // y_max < m * (n + 1)
        // floor(y_max / m) <= n
        n = y_max / m;
        b = y_max % m;
        std::mem::swap(&mut m, &mut a);
    }
    ans
}

pub fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    debug_assert!(0 <= n && n < (1 << 32));
    debug_assert!(1 <= m && m < (1 << 32));
    let mut ans = 0;
    if a < 0 {
        let a2 = safe_mod(a, m);
        ans -= (n * (n - 1) / 2) as u64 * ((a2 - a) / m) as u64;
        a = a2;
    }
    if b < 0 {
        let b2 = safe_mod(b, m);
        ans -= n as u64 * ((b2 - b) / m) as u64;
        b = b2;
    }
    (ans + floor_sum_unsigned(n as u64, m as u64, a as u64, b as u64)) as i64
}
