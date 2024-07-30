use super::mod_int::{MOD, ModInt, primitive_root_const};

const G: u32 = primitive_root_const(MOD as i32) as u32;
const RANK2: usize = (MOD - 1).trailing_zeros() as usize;

struct FftInfo {
    root: [ModInt; RANK2 + 1],
    iroot: [ModInt; RANK2 + 1],
    rate2: [ModInt; RANK2 - 2 + 1],
    irate2: [ModInt; RANK2 - 2 + 1],
    rate3: [ModInt; RANK2 - 3 + 1],
    irate3: [ModInt; RANK2 - 3 + 1],
}

impl FftInfo {
    fn new() -> Self {
        let mut root = [ModInt::zero(); RANK2 + 1];
        root[RANK2] = ModInt::raw(G).pow(MOD - 1 >> RANK2);
        let mut iroot = [ModInt::zero(); RANK2 + 1];
        iroot[RANK2] = root[RANK2].inv();
        for i in (0..RANK2).rev() {
            root[i] = root[i + 1] * root[i + 1];
            iroot[i] = iroot[i + 1] * iroot[i + 1];
        }
        let mut rate2 = [ModInt::zero(); RANK2 - 1];
        let mut irate2 = [ModInt::zero(); RANK2 - 1];
        let mut prod = ModInt::one();
        let mut iprod = ModInt::one();
        for i in 0..=(RANK2 - 2) {
            rate2[i] = root[i + 2] * prod;
            irate2[i] = iroot[i + 2] * iprod;
            prod *= iroot[i + 2];
            iprod *= root[i + 2];
        }
        let mut rate3 = [ModInt::zero(); RANK2 - 2];
        let mut irate3 = [ModInt::zero(); RANK2 - 2];
        let mut prod = ModInt::one();
        let mut iprod = ModInt::one();
        for i in 0..=(RANK2 - 3) {
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

fn butterfly(a: &mut Vec<ModInt>, info: &FftInfo) {
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
                    let a1na3imag = ModInt::from_u64(a1 + mod2 - a3).val() as u64 * imag.val() as u64;
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

fn butterfly_inv(a: &mut Vec<ModInt>, info: &FftInfo) {
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
                    a[i + offset + p] = ModInt::from_u64((MOD + l.val() as i64 - r.val() as i64) as u64 * irot.val() as u64);
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
                    let a2na3iimag = ModInt::from_u64((MOD as u64 + a2 - a3) * iimag.val() as u64).val() as u64;
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

fn convolution_naive(a: &[ModInt], b: &[ModInt]) -> Vec<ModInt> {
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

fn convolution_fft(mut a: Vec<ModInt>, mut b: Vec<ModInt>) -> Vec<ModInt> {
    let n = a.len();
    let m = b.len();
    let z = 1 << (32 - ((n + m - 1).saturating_sub(1) as u32).leading_zeros());  // bit ceil

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

pub fn convolution(a: &[ModInt], b: &[ModInt]) -> Vec<ModInt> {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return Vec::new();
    }
    let z = 1 << (32 - ((n + m - 1).saturating_sub(1) as u32).leading_zeros()); // bit ceil
    debug_assert!((MOD - 1) % z == 0);

    if n.min(m) <= 60 {
        convolution_naive(a, b)
    } else {
        convolution_fft(a.to_vec(), b.to_vec())
    }
}
