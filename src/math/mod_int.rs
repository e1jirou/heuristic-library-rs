pub const MOD: i64 = 998244353;

const UMOD: u32 = MOD as u32;
const PRIME: bool = is_prime_const(UMOD as i32);

#[derive(Clone, Copy)]
pub struct ModInt {
    v: u32,
}

impl ModInt {
    pub fn from_i64(v: i64) -> Self {
        ModInt {
            v: safe_mod(v, MOD) as u32,
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
        if PRIME {
            debug_assert!(self.v > 0);
            self.pow(MOD - 2)
        } else {
            let eg = inv_gcd(self.v as i64, MOD);
            debug_assert!(eg.0 == 1);
            Self::from_i64(eg.1)
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
            Self::zero()
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

impl Default for ModInt {
    fn default() -> Self {
        ModInt::zero()
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
