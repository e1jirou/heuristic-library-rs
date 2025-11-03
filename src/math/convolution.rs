use super::mod_int::{ModInt, inv_gcd, primitive_root_const, safe_mod};
use num_traits::{Zero, One};

const MAX_RANK2: usize = 31;

struct FftInfo<const MOD: u32> {
    root: [ModInt<MOD>; MAX_RANK2 + 1],
    iroot: [ModInt<MOD>; MAX_RANK2 + 1],
    rate2: [ModInt<MOD>; MAX_RANK2 - 2 + 1],
    irate2: [ModInt<MOD>; MAX_RANK2 - 2 + 1],
    rate3: [ModInt<MOD>; MAX_RANK2 - 3 + 1],
    irate3: [ModInt<MOD>; MAX_RANK2 - 3 + 1],
}

impl<const MOD: u32> FftInfo<MOD> {
    fn new() -> Self {
        let rank2: usize = (MOD - 1).trailing_zeros() as usize;
        let mut root = [ModInt::zero(); MAX_RANK2 + 1];
        let g: u32 = primitive_root_const(MOD as i32) as u32;
        root[rank2] = ModInt::raw(g).pow(MOD as i64 - 1 >> rank2);
        let mut iroot = [ModInt::zero(); MAX_RANK2 + 1];
        iroot[rank2] = root[rank2].inv();
        for i in (0..rank2).rev() {
            root[i] = root[i + 1] * root[i + 1];
            iroot[i] = iroot[i + 1] * iroot[i + 1];
        }
        let mut rate2 = [ModInt::zero(); MAX_RANK2 - 1];
        let mut irate2 = [ModInt::zero(); MAX_RANK2 - 1];
        let mut prod = ModInt::one();
        let mut iprod = ModInt::one();
        for i in 0..=(rank2 - 2) {
            rate2[i] = root[i + 2] * prod;
            irate2[i] = iroot[i + 2] * iprod;
            prod *= iroot[i + 2];
            iprod *= root[i + 2];
        }
        let mut rate3 = [ModInt::zero(); MAX_RANK2 - 2];
        let mut irate3 = [ModInt::zero(); MAX_RANK2 - 2];
        let mut prod = ModInt::one();
        let mut iprod = ModInt::one();
        for i in 0..=(rank2 - 3) {
            rate3[i] = root[i + 3] * prod;
            irate3[i] = iroot[i + 3] * iprod;
            prod *= iroot[i + 3];
            iprod *= root[i + 3];
        }
        FftInfo {
            root,
            iroot,
            rate2,
            irate2,
            rate3,
            irate3,
        }
    }
}

fn butterfly<const MOD: u32>(a: &mut Vec<ModInt<MOD>>, info: &FftInfo<MOD>) {
    let n = a.len();
    let h = n.trailing_zeros() as i32;

    let mut len = 0;  // a[i, i+(n>>len), i+2*(n>>len), ..] is transformed
    while len < h {
        if h - len == 1 {
            let p = 1 << (h - len - 1);
            let mut rot = ModInt::one();
            for s in 0..(1 << len) {
                let offset = s << (h - len);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p] * rot;
                    a[i + offset] = l + r;
                    a[i + offset + p] = l - r;
                }
                if s + 1 != (1 << len) {
                    rot *= info.rate2[(!(s as u32)).trailing_zeros() as usize];
                }
            }
            len += 1;
        } else {
            // 4-base
            let p = 1 << (h - len - 2);
            let mut rot = ModInt::one();
            let imag = info.root[2];
            for s in 0..(1 << len) {
                let rot2 = rot * rot;
                let rot3 = rot2 * rot;
                let offset = s << (h - len);
                for i in 0..p {
                    let mod2 = 1 * (MOD as u64) * (MOD as u64);
                    let a0 = a[i + offset].val() as u64;
                    let a1 = a[i + offset + p].val() as u64 * rot.val() as u64;
                    let a2 = a[i + offset + 2 * p].val() as u64 * rot2.val() as u64;
                    let a3 = a[i + offset + 3 * p].val() as u64 * rot3.val() as u64;
                    let a1na3imag = ModInt::<MOD>::from_u64(a1 + mod2 - a3).val() as u64 * imag.val() as u64;
                    let na2 = mod2 - a2;
                    a[i + offset] = ModInt::from_u64(a0 + a2 + a1 + a3);
                    a[i + offset + p] = ModInt::from_u64(a0 + a2 + (2 * mod2 - (a1 + a3)));
                    a[i + offset + 2 * p] = ModInt::from_u64(a0 + na2 + a1na3imag);
                    a[i + offset + 3 * p] = ModInt::from_u64(a0 + na2 + (mod2 - a1na3imag));
                }
                if s + 1 != (1 << len) {
                    rot *= info.rate3[(!(s as u32)).trailing_zeros() as usize];
                }
            }
            len += 2;
        }
    }
}

fn butterfly_inv<const MOD: u32>(a: &mut Vec<ModInt<MOD>>, info: &FftInfo<MOD>) {
    let n = a.len();
    let h = n.trailing_zeros() as i32;

    let mut len = h;  // a[i, i+(n>>len), i+2*(n>>len), ..] is transformed
    while len != 0 {
        if len == 1 {
            let p = 1 << (h - len);
            let mut irot = ModInt::one();
            for s in 0..(1 << (len - 1)) {
                let offset = s << (h - len + 1);
                for i in 0..p {
                    let l = a[i + offset];
                    let r = a[i + offset + p];
                    a[i + offset] = l + r;
                    a[i + offset + p] = ModInt::from_u64((MOD as i64 + l.val() as i64 - r.val() as i64) as u64 * irot.val() as u64);
                }
                if s + 1 != (1 << (len - 1)) {
                    irot *= info.irate2[(!(s as u32)).trailing_zeros() as usize];
                }
            }
            len -= 1;
        } else {
            // 4-base
            let p = 1 << (h - len);
            let mut irot = ModInt::one();
            let iimag = info.iroot[2];
            for s in 0..(1 << (len - 2)) {
                let irot2 = irot * irot;
                let irot3 = irot2 * irot;
                let offset = s << (h - len + 2);
                for i in 0..p {
                    let a0 = a[i + offset].val() as u64;
                    let a1 = a[i + offset + p].val() as u64;
                    let a2 = a[i + offset + 2 * p].val() as u64;
                    let a3 = a[i + offset + 3 * p].val() as u64;
                    let a2na3iimag = ModInt::<MOD>::from_u64((MOD as u64 + a2 - a3) * iimag.val() as u64).val() as u64;
                    a[i + offset] = ModInt::from_u64(a0 + a1 + a2 + a3);
                    a[i + offset + p] = ModInt::from_u64((a0 + (MOD as u64 - a1) + a2na3iimag) * irot.val() as u64);
                    a[i + offset + 2 * p] = ModInt::from_u64((a0 + a1 + (MOD as u64 - a2) + (MOD as u64 - a3)) * irot2.val() as u64);
                    a[i + offset + 3 * p] = ModInt::from_u64((a0 + (MOD as u64 - a1) + (MOD as u64 - a2na3iimag)) * irot3.val() as u64);
                }
                if s + 1 != (1 << (len - 2)) {
                    irot *= info.irate3[(!(s as u32)).trailing_zeros() as usize];
                }
            }
            len -= 2;
        }
    }
}

fn convolution_naive<const MOD: u32>(a: &[ModInt<MOD>], b: &[ModInt<MOD>]) -> Vec<ModInt<MOD>> {
    let n = a.len();
    let m = b.len();
    let mut ans = vec![ModInt::zero(); n + m - 1];
    if n < m {
        for j in 0..m {
            for i in 0..n {
                ans[i + j] += a[i] * b[j];
            }
        }
    } else {
        for i in 0..n {
            for j in 0..m {
                ans[i + j] += a[i] * b[j];
            }
        }
    }
    ans
}

fn convolution_fft<const MOD: u32>(mut a: Vec<ModInt<MOD>>, mut b: Vec<ModInt<MOD>>) -> Vec<ModInt<MOD>> {
    let n = a.len();
    let m = b.len();
    let z = (n + m - 1).next_power_of_two();

    let info = FftInfo::new();

    a.resize(z, ModInt::zero());
    butterfly(&mut a, &info);
    b.resize(z, ModInt::zero());
    butterfly(&mut b, &info);
    for i in 0..z {
        a[i] *= b[i];
    }
    butterfly_inv(&mut a, &info);
    a.truncate(n + m - 1);
    let iz = ModInt::from_usize(z).inv();
    for i in 0..(n + m - 1) {
        a[i] *= iz;
    }
    a
}

pub fn convolution<const MOD: u32>(a: Vec<ModInt<MOD>>, b: Vec<ModInt<MOD>>) -> Vec<ModInt<MOD>> {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }
    let z = (n + m - 1).next_power_of_two();
    debug_assert_eq!((MOD as usize - 1) % z, 0);

    if n.min(m) <= 60 {
        convolution_naive(&a, &b)
    } else {
        convolution_fft(a, b)
    }
}

fn convolution_mod_i64<const MOD: u32>(a: &[i64], b: &[i64]) -> Vec<i64> {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }
    let z = (n + m - 1).next_power_of_two();
    debug_assert_eq!((MOD as usize - 1) % z, 0);

    let mut a2 = vec![ModInt::zero(); n];
    for i in 0..n {
        a2[i] = ModInt::from_i64(a[i]);
    }
    let mut b2 = vec![ModInt::zero(); m];
    for i in 0..m {
        b2[i] = ModInt::from_i64(b[i]);
    }
    let c2 = convolution::<MOD>(a2, b2);
    let mut c = vec![0; n + m - 1];
    for i in 0..(n + m - 1) {
        c[i] = c2[i].val() as i64;
    }
    c
}

pub fn convolution_i64(a: &[i64], b: &[i64]) -> Vec<i64> {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }

    const MOD1: u64 = 754974721; // 2^24
    const MOD2: u64 = 167772161; // 2^25
    const MOD3: u64 = 469762049; // 2^26
    const M2M3: u64 = MOD2 * MOD3;
    const M1M3: u64 = MOD1 * MOD3;
    const M1M2: u64 = MOD1 * MOD2;
    const M1M2M3: u64 = (MOD1 * MOD2).wrapping_mul(MOD3);

    const I1: u64 = inv_gcd((MOD2 * MOD3) as i64, MOD1 as i64).1 as u64;
    const I2: u64 = inv_gcd((MOD1 * MOD3) as i64, MOD2 as i64).1 as u64;
    const I3: u64 = inv_gcd((MOD1 * MOD2) as i64, MOD3 as i64).1 as u64;

    const MAX_AB_BIT: usize = 24;

    debug_assert_eq!(MOD1 % (1 << MAX_AB_BIT), 1, "MOD1 isn't enough to support an array length of 2^24.");
    debug_assert_eq!(MOD2 % (1 << MAX_AB_BIT), 1, "MOD2 isn't enough to support an array length of 2^24.");
    debug_assert_eq!(MOD3 % (1 << MAX_AB_BIT), 1, "MOD3 isn't enough to support an array length of 2^24.");
    debug_assert!(n + m - 1 <= (1 << MAX_AB_BIT));

    let c1 = convolution_mod_i64::<754974721>(a, b);
    let c2 = convolution_mod_i64::<167772161>(a, b);
    let c3 = convolution_mod_i64::<469762049>(a, b);

    let mut c = vec![0; n + m - 1];
    for i in 0..(n + m - 1) {
        let mut x: u64 = 0;
        x = x.wrapping_add(((c1[i] as u64 * I1) % MOD1).wrapping_mul(M2M3));
        x = x.wrapping_add(((c2[i] as u64 * I2) % MOD2).wrapping_mul(M1M3));
        x = x.wrapping_add(((c3[i] as u64 * I3) % MOD3).wrapping_mul(M1M2));
        // B = 2^63, -B <= x, r(real value) < B
        // (x, x - M, x - 2M, or x - 3M) = r (mod 2B)
        // r = c1[i] (mod MOD1)
        // focus on MOD1
        // r = x, x - M', x - 2M', x - 3M' (M' = M % 2^64) (mod 2B)
        // r = x,
        //     x - M' + (0 or 2B),
        //     x - 2M' + (0, 2B or 4B),
        //     x - 3M' + (0, 2B, 4B or 6B) (without mod!)
        // (r - x) = 0, (0)
        //           - M' + (0 or 2B), (1)
        //           -2M' + (0 or 2B or 4B), (2)
        //           -3M' + (0 or 2B or 4B or 6B) (3) (mod MOD1)
        // we checked that
        //   ((1) mod MOD1) mod 5 = 2
        //   ((2) mod MOD1) mod 5 = 3
        //   ((3) mod MOD1) mod 5 = 4
        let mut diff = c1[i] - safe_mod(x as i64, MOD1 as i64);
        if diff < 0 {
            diff += MOD1 as i64;
        }
        const OFFSET: [u64; 5] = [0, 0, M1M2M3, 2 * M1M2M3, 3 * M1M2M3];
        c[i] = x.wrapping_sub(OFFSET[diff as usize % 5]) as i64;
    }

    c
}
