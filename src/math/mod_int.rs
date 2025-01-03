#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct ModInt<const MOD: u32> {
    v: u32,
}

impl<const MOD: u32> ModInt<MOD> {
    pub fn from_i64(v: i64) -> Self {
        ModInt {
            v: safe_mod(v, MOD as i64) as u32,
        }
    }

    pub fn from_u64(v: u64) -> Self {
        ModInt {
            v: (v % (MOD as u64)) as u32,
        }
    }

    pub fn from_usize(v: usize) -> Self {
        ModInt {
            v: (v % (MOD as usize)) as u32,
        }
    }

    pub fn zero() -> Self {
        Self::raw(0)
    }

    pub fn one() -> Self {
        Self::raw(1)
    }

    pub fn raw(v: u32) -> Self {
        debug_assert!(v < MOD);
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
        let mut r = Self::one();
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
        if is_prime_const(MOD as i32) {
            debug_assert!(self.v > 0);
            self.pow(MOD as i64 - 2)
        } else {
            let eg = inv_gcd(self.v as i64, MOD as i64);
            debug_assert!(eg.0 == 1);
            Self::from_i64(eg.1)
        }
    }
}

impl<const MOD: u32> std::ops::AddAssign for ModInt<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        self.v += rhs.v;
        if self.v >= MOD {
            self.v -= MOD;
        }
    }
}

impl<const MOD: u32> std::ops::SubAssign for ModInt<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.v < rhs.v {
            self.v += MOD;
        }
        self.v -= rhs.v;
    }
}

impl<const MOD: u32> std::ops::MulAssign for ModInt<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        self.v = (self.v as u64 * rhs.v as u64 % MOD as u64) as u32;
    }
}

impl<const MOD: u32> std::ops::DivAssign for ModInt<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}

impl<const MOD: u32> std::ops::Neg for ModInt<MOD> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.v == 0 {
            Self::zero()
        } else {
            Self::raw(MOD - self.v)
        }
    }
}

impl<const MOD: u32> std::ops::Add for ModInt<MOD> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<const MOD: u32> std::ops::Sub for ModInt<MOD> {
    type Output = Self;
    
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const MOD: u32> std::ops::Mul for ModInt<MOD> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<const MOD: u32> std::ops::Div for ModInt<MOD> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<const MOD: u32> Default for ModInt<MOD> {
    fn default() -> Self {
        ModInt::zero()
    }
}

impl<const MOD: u32> PartialEq for ModInt<MOD> {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

impl<const MOD: u32> Eq for ModInt<MOD> {}

impl<const MOD: u32> std::fmt::Display for ModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.v)
    }
}

// return x mod m
pub const fn safe_mod(mut x: i64, m: i64) -> i64 {
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
        i += 1;
        let mut t = d;
        let mut y = pow_mod_const(a, t, n);
        while t != n as i64 - 1 && y != 1 && y != n as i64 - 1 {
            y = y * y % n as i64;
            t <<= 1;
        }
        if y != n as i64 - 1 && t % 2 == 0 {
            return false;
        }
    }
    return true;
}

// return pair(g, x) s.t. g = gcd(a, b), xa = g (mod b), 0 <= x < b/g
pub const fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
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

pub const fn primitive_root_const(m: i32) -> i32 {
    if m == 2 {
        return 1;
    }
    if m == 167772161 {
        return 3;
    }
    if m == 469762049 {
        return 3;
    }
    if m == 754974721 {
        return 11;
    }
    if m == 998244353 {
        return 3;
    }
    let mut divs = [0; 20];
    divs[0] = 2;
    let mut cnt = 1;
    let mut x = (m - 1) / 2;
    while x % 2 == 0 {
        x /= 2;
    }
    let mut i = 3;
    while i * i <= x {
        if x % i == 0 {
            divs[cnt] = i;
            cnt += 1;
            while x % i == 0 {
                x /= i;
            }
        }
        i += 2;
    }
    if x > 1 {
        divs[cnt] = x;
        cnt += 1;
    }
    let mut g = 2;
    loop {
        let mut ok = true;
        let mut i = 0;
        while i < cnt {
            if pow_mod_const(g, ((m - 1) / divs[i]) as i64, m) == 1 {
                ok = false;
                break;
            }
            i += 1;
        }
        if ok {
            return g as i32;
        }
        g += 1;
    }
}
