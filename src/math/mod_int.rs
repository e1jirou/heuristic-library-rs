// return x mod m
const fn safe_mod(mut x: i64, m: i64) -> i64 {
    debug_assert!(m >= 1);
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}

// return `(x ** n) % m`
pub const fn pow_mod_const(x: i64, mut n: i64, m: i32) -> i64 {
    debug_assert!(n >= 0);
    debug_assert!(m >= 1);
    if m == 1 {
        return 0;
    }
    let mut r = 1;
    let mut y = safe_mod(x, m as i64) as u64;
    while n != 0 {
        if (n & 1) > 0 {
            r = (r * y) % (m as u64);
        }
        y = (y * y) % (m as u64);
        n >>= 1;
    }
    r as i64
}

// Reference:
// M. Forisek and J. Jancina,
// Fast Primality Testing for Integers That Fit into a Machine Word
// @param n `0 <= n`
const fn is_prime_const(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 7 || n == 61 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut d = n as i64 - 1;
    while d % 2 == 0 {
        d /= 2;
    }
    let bases = [2, 7, 61];
    let mut i = 0;
    while i < 3 {
        let a = bases[i];
        let mut t = d;
        let mut y = pow_mod_const(a, t, n);
        while t != n as i64 - 1 && y != 1 && y != n as i64 - 1 {
            y = y * y % n as i64;
            t <<= 1;
        }
        if y != n as i64 - 1 && t % 2 == 0 {
            return false;
        }
        i += 1
    }
    return true;
}

// return pair(g, x) s.t. g = gcd(a, b), xa = g (mod b), 0 <= x < b/g
const fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
    debug_assert!(b >= 1);
    let a = safe_mod(a, b);
    if a == 0 {
        return (b, 0);
    }
    // Contracts:
    // [1] s - m0 * a = 0 (mod b)
    // [2] t - m1 * a = 0 (mod b)
    // [3] s * |m1| + t * |m0| <= b
    let mut s = b;
    let mut t = a;
    let mut m0 = 0;
    let mut m1 = 1;

    while t != 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u; // |m1 * u| <= |m1| * s <= b

        // [3]:
        // (s - t * u) * |m1| + t * |m0 - m1 * u|
        // <= s * |m1| - t * u * |m1| + t * (|m0| + |m1| * u)
        // = s * |m1| + t * |m0| <= b

        let mut tmp = s;
        s = t;
        t = tmp;
        tmp = m0;
        m0 = m1;
        m1 = tmp;
    }
    // by [3]: |m0| <= b/g
    // by g != b: |m0| < b/g
    if m0 < 0 {
        m0 += b / s;
    }
    (s, m0)
}

pub const MOD: i64 = 998244353;

const UMOD: u32 = MOD as u32;
const PRIME: bool = is_prime_const(UMOD as i32);

#[derive(Clone, Copy)]
pub struct ModInt {
    v: u32,
}

impl ModInt {
    pub fn new(v: i64) -> Self {
        ModInt {
            v: safe_mod(v, MOD) as u32,
        }
    }

    pub fn raw(v: u32) -> Self {
        debug_assert!(v < UMOD);
        ModInt {
            v,
        }
    }

    pub fn val(&self) -> u32 {
        self.v
    }

    pub fn pow(&self, mut n: i64) -> Self {
        debug_assert!(n >= 0);
        let mut x = *self;
        let mut r = Self::raw(1);
        while n > 0 {
            if (n & 1) == 1 {
                r *= x;
            }
            x *= x;
            n >>= 1;
        }
        r
    }

    pub fn inv(&self) -> Self {
        if PRIME {
            debug_assert!(self.v > 0);
            self.pow(MOD - 2)
        } else {
            let eg = inv_gcd(self.v as i64, MOD);
            debug_assert!(eg.0 == 1);
            Self::new(eg.1)
        }
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        self.v += rhs.v;
        if self.v >= UMOD {
            self.v -= UMOD;
        }
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        if self.v < rhs.v {
            self.v += UMOD;
        }
        self.v -= rhs.v;
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.v = (self.v as u64 * rhs.v as u64 % MOD as u64) as u32;
    }
}

impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}

impl std::ops::Neg for ModInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.v == 0 {
            Self::raw(0)
        } else {
            Self::raw(UMOD - self.v)
        }
    }
}

impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;
    
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl std::ops::Div for ModInt {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl PartialEq for ModInt {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl Eq for ModInt {}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.v)
    }
}
