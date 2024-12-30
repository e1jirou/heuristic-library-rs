// size: bytes
// align: bytes
// return empty vector
pub fn aligned_alloc<T>(size: usize, align: usize) -> Vec<T> {
    let capacity = size / size_of::<T>();
    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
        let ptr = std::alloc::alloc(layout);
        Vec::from_raw_parts(ptr as *mut T, 0, capacity)
    }
}

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

const fn get_pr(mod_: u32) -> u32 {
    if mod_ == 2 {
        return 1;
    }
    let mut ds = [0; 32];
    let mut idx = 0;
    let mut m = mod_ as u64 - 1;
    let mut i = 2;
    while i * i <= m {
        if m % i == 0 {
            ds[idx] = i;
            idx += 1;
            while m % i == 0 {
                m /= i;
            }
        }
        i += 1;
    }
    if m != 1 {
        ds[idx] = m;
        idx += 1;
    }
    let mut pr = 2;
    loop {
        let mut flg = true;
        let mut i = 0;
        while i < idx {
            let mut a = pr as u64;
            let mut b = (mod_ as u64 - 1) / ds[i];
            let mut r = 1;
            while b > 0 {
                if b & 1 == 1 {
                    r = r * a % mod_ as u64;
                }
                a = a * a % mod_ as u64;
                b >>= 1;
            }
            if r == 1 {
                flg = false;
                break;
            }
            i += 1;
        }
        if flg {
            break;
        }
        pr += 1;
    }
    pr
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
            pr: get_pr(MOD),
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

    pub fn ntt(&self, a: &mut Vec<MontgomeryModInt<MOD>>, n: usize) {
        if n <= 1 {
            return;
        }
        let k = n.trailing_zeros();
        if k == 1 {
            let a1 = a[1];
            a[1] = a[0] - a1;
            a[0] = a[0] + a1;
            return;
        }
        // 2-base
        if k & 1 == 1 {
            let v = 1 << (k - 1);
            for j in 0..v {
                let aj = a[j];
                let ajv = a[j + v];
                a[j] = aj + ajv;
                a[j + v] = aj - ajv;
            }
        }
        // 4-base
        let mut u = 1 << (2 + (k & 1));
        let mut v = 1 << (k - 2 - (k & 1));
        let one = MontgomeryModInt::one();
        let imag = self.dw[1];
        while v > 0 {
            let mut xx = one;
            let mut jh = 0;
            while jh < u {
                let ww = xx * xx;
                let wx = ww * xx;
                let mut j0 = jh * v;
                let mut j1 = j0 + v;
                let mut j2 = j1 + v;
                let mut j3 = j2 + v;
                let je = j1;
                while j0 < je {
                    let t0 = a[j0];
                    let t1 = a[j1] * xx;
                    let t2 = a[j2] * ww;
                    let t3 = a[j3] * wx;
                    let t0p2 = t0 + t2;
                    let t1p3 = t1 + t3;
                    let t0m2 = t0 - t2;
                    let t1m3 = (t1 - t3) * imag;
                    a[j0] = t0p2 + t1p3;
                    a[j1] = t0p2 - t1p3;
                    a[j2] = t0m2 + t1m3;
                    a[j3] = t0m2 - t1m3;
                    j0 += 8;
                    j1 += 8;
                    j2 += 8;
                    j3 += 8;
                }
                jh += 4;
                xx *= self.dw[jh.trailing_zeros() as usize];
            }
            u <<= 2;
            v >>= 2;
        }
    }

    pub fn intt(&self, a: &mut Vec<MontgomeryModInt<MOD>>, n: usize) {
        if n <= 1 {
            return;
        }
        let k = n.trailing_zeros();
        if k == 1 {
            let a1 = a[1];
            a[1] = a[0] - a1;
            a[0] = a[0] + a1;
            return;
        }
        // 4-base
        let mut u = 1 << (k - 2);
        let mut v = 1;
        let one = MontgomeryModInt::one();
        let imag = self.dy[1];
        while u > 0 {
            u <<= 2;
            let mut xx = one;
            let mut jh = 0;
            while jh < u {
                let ww = xx * xx;
                let yy = xx * imag;
                let mut j0 = jh * v;
                let mut j1 = j0 + v;
                let mut j2 = j1 + v;
                let mut j3 = j2 + v;
                let je = j1;
                while j0 < je {
                    let t0 = a[j0];
                    let t1 = a[j1];
                    let t2 = a[j2];
                    let t3 = a[j3];
                    let t0p1 = t0 + t1;
                    let t2p3 = t2 + t3;
                    let t0m1 = (t0 - t1) * xx;
                    let t2m3 = (t2 - t3) * yy;
                    a[j0] = t0p1 + t2p3;
                    a[j1] = (t0p1 - t2p3) * ww;
                    a[j2] = t0m1 + t2m3;
                    a[j3] = (t0m1 - t2m3) * ww;
                    j0 += 8;
                    j1 += 8;
                    j2 += 8;
                    j3 += 8;
                }
                jh += 4;
                xx *= self.dy[jh.trailing_zeros() as usize];
            }
            u >>= 4;
            v <<= 2;
        }
        // 2-base
        if k & 1 == 1 {
            let v = 1 << (k - 1);
            for j in 0..v {
                let aj = a[j];
                let ajv = a[j + v];
                a[j] = aj + ajv;
                a[j + v] = aj - ajv;
            }
        }
        todo!();
    }

    // write result to a
    pub fn multiply(&self, a: &mut Vec<MontgomeryModInt<MOD>>, b: &mut Vec<MontgomeryModInt<MOD>>) {
        if a.is_empty() || b.is_empty() {
            a.clear();
            return;
        }
        let l = a.len() + b.len() - 1;
        if a.len().min(b.len()) <= 40 {
            let mut s = vec![MontgomeryModInt::zero(); l];
            for i in 0..a.len() {
                for j in 0..b.len() {
                    s[i + j] += a[i] * b[j];
                }
            }
            std::mem::swap(a, &mut s);
            return;
        }
        let n = l.next_power_of_two();
        a.resize(n, MontgomeryModInt::zero());
        self.ntt(a, n);
        b.resize(n, MontgomeryModInt::zero());
        self.ntt(b, n);
        for i in 0..n {
            a[i] *= b[i];
        }
        self.intt(a, n);
        let invn = MontgomeryModInt::from_i64(n as i64).inv();
        for i in 0..n {
            a[i] *= invn;
        }
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
