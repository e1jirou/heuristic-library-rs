#[derive(Clone, Copy, Debug)]
pub struct MontgomeryModInt<const MOD: u32> {
    pub v: u32,
}

impl<const MOD: u32> MontgomeryModInt<MOD> {
    const fn r() -> u32 {
        let mut ret = MOD;
        let mut i = 0;
        while i < 4 {
            ret = ret.wrapping_mul((2 as u32).wrapping_sub(MOD.wrapping_mul(ret)));
            i += 1;
        }
        ret
    }

    const fn n2() -> u32 {
        (-(MOD as i64) as u64 % MOD as u64) as u32
    }

    pub const fn reduce(b: u64) -> u32 {
        ((b + ((b as u32).wrapping_mul(-(Self::r() as i32) as u32)) as u64 * MOD as u64) >> 32) as u32
    }

    pub const fn from_i64(v: i64) -> Self {
        MontgomeryModInt {
            v: Self::reduce((v % MOD as i64 + MOD as i64) as u64 * Self::n2() as u64)
        }
    }

    pub const fn zero() -> Self {
        Self { v: 0 }
    }

    pub const fn one() -> Self {
        Self::from_i64(1)
    }

    pub const fn val(&self) -> u32 {
        let ret = Self::reduce(self.v as u64);
        if ret >= MOD {ret - MOD} else {ret}
    }

    pub fn pow(&self, mut n: u64) -> Self {
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

    pub const fn inv(&self) -> Self {
        let mut x = self.val() as i32;
        let mut y = MOD as i32;
        let mut u = 1;
        let mut v = 0;
        let mut t = 0;
        let mut tmp = 0;
        while y > 0 {
            t = x / y;
            x -= t * y;
            u -= t * v;
            tmp = x;
            x = y;
            y = tmp;
            tmp = u;
            u = v;
            v = tmp;
        }
        Self::from_i64(u as i64)
    }
}

impl<const MOD: u32> std::ops::AddAssign for MontgomeryModInt<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        self.v += rhs.v;
        if self.v >= 2 * MOD {
            self.v -= 2 * MOD;
        }
    }
}

impl<const MOD: u32> std::ops::SubAssign for MontgomeryModInt<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        if self.v < rhs.v {
            self.v += 2 * MOD;
        }
        self.v -= rhs.v;
    }
}

impl<const MOD: u32> std::ops::MulAssign for MontgomeryModInt<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        self.v = Self::reduce(self.v as u64 * rhs.v as u64);
    }
}

impl<const MOD: u32> std::ops::DivAssign for MontgomeryModInt<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}

impl<const MOD: u32> std::ops::Neg for MontgomeryModInt<MOD> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::zero() - self
    }
}

impl<const MOD: u32> std::ops::Add for MontgomeryModInt<MOD> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self += rhs;
        self
    }
}

impl<const MOD: u32> std::ops::Sub for MontgomeryModInt<MOD> {
    type Output = Self;
    
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const MOD: u32> std::ops::Mul for MontgomeryModInt<MOD> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<const MOD: u32> std::ops::Div for MontgomeryModInt<MOD> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<const MOD: u32> Default for MontgomeryModInt<MOD> {
    fn default() -> Self {
        MontgomeryModInt::zero()
    }
}

impl<const MOD: u32> PartialEq for MontgomeryModInt<MOD> {
    fn eq(&self, other: &Self) -> bool {
        (if self.v >= MOD {self.v - MOD} else {self.v}) == (if other.v >= MOD {other.v - MOD} else {other.v})
    }
}

impl<const MOD: u32> Eq for MontgomeryModInt<MOD> {}

impl<const MOD: u32> std::fmt::Display for MontgomeryModInt<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val())
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

pub struct NTT<const MOD: u32> {
    pr: u32,
    level: usize,
    dw: Vec<MontgomeryModInt<MOD>>,
    dy: Vec<MontgomeryModInt<MOD>>,
}

impl<const MOD: u32> NTT<MOD> {
    pub fn new() -> Self {
        let level = (MOD - 1).trailing_zeros() as usize;
        let mut ret = Self {
            pr: primitive_root_const(MOD as i32) as u32,
            level,
            dw: vec![MontgomeryModInt::zero(); level],
            dy: vec![MontgomeryModInt::zero(); level],
        };
        ret.setwy();
        ret
    }

    fn setwy(&mut self) {
        let mut w = vec![MontgomeryModInt::zero(); self.level];
        let mut y = vec![MontgomeryModInt::zero(); self.level];
        w[self.level - 1] = MontgomeryModInt::from_i64(self.pr as i64).pow(((MOD - 1) / (1 << self.level)) as u64);
        y[self.level - 1] = w[self.level - 1].inv();
        for i in (1..(self.level - 2)).rev() {
            w[i] = w[i + 1] * w[i + 1];
            y[i] = y[i + 1] * y[i + 1];
        }
        self.dw[0] = w[1] * w[1];
        self.dy[0] = self.dw[0];
        self.dw[1] = w[1];
        self.dy[1] = y[1];
        self.dw[2] = w[2];
        self.dy[2] = y[2];
        for i in 3..self.level {
            self.dw[i] = self.dw[i - 1] * y[i - 2] * w[i];
            self.dy[i] = self.dy[i - 1] * w[i - 2] * w[i];
        }
    }

    pub fn ntt(a: &mut Vec<u32>, n: usize) {
        todo!();
    }

    pub fn intt(a: &mut Vec<u32>, n: usize) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::MontgomeryModInt;

    #[test]
    fn montgomery_test() {
        const MOD: u32 = 998244353;
        let one = MontgomeryModInt::<MOD>::from_i64(1);
        let two = MontgomeryModInt::<MOD>::from_i64(2);
        let three = MontgomeryModInt::<MOD>::from_i64(3);
        debug_assert_eq!(-two, MontgomeryModInt::<MOD>::from_i64(-2));
        debug_assert_eq!(one + two, three);
        debug_assert_eq!(three - one, two);
        debug_assert_eq!((two * three).val(), 6);
        debug_assert_eq!((three / two * two).val(), 3);
        debug_assert_eq!(three.pow(3).val(), 27);
    }
}
