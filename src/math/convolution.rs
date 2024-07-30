use super::mod_int::{MOD, ModInt, pow_mod_const};

const fn primitive_root_const(m: i32) -> i32 {
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

const G: u32 = primitive_root_const(MOD as i32) as u32;
const RANK2: usize = 63 - ((MOD - 1) & (1 - MOD)).leading_zeros() as usize;

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
        let mut root = [ModInt::default(); RANK2 + 1];
        root[RANK2] = ModInt::raw(G).pow(MOD - 1 >> RANK2);
        let mut iroot = [ModInt::default(); RANK2 + 1];
        iroot[RANK2] = root[RANK2].inv();
        for i in (0..RANK2).rev() {
            root[i] = root[i + 1] * root[i + 1];
            iroot[i] = iroot[i + 1] * iroot[i + 1];
        }
        let mut rate2 = [ModInt::default(); RANK2 - 1];
        let mut irate2 = [ModInt::default(); RANK2 - 1];
        let mut prod = ModInt::raw(1);
        let mut iprod = ModInt::raw(1);
        for i in 0..=(RANK2 - 2) {
            rate2[i] = root[i + 2] * prod;
            irate2[i] = iroot[i + 2] * iprod;
            prod *= iroot[i + 2];
            iprod *= root[i + 2];
        }
        let mut rate3 = [ModInt::default(); RANK2 - 2];
        let mut irate3 = [ModInt::default(); RANK2 - 2];
        let mut prod = ModInt::raw(1);
        let mut iprod = ModInt::raw(1);
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

fn butterfly(a: &mut Vec<i64>, info: &FftInfo) {
    todo!();
}

fn butterfly_inv(a: &mut Vec<i64>, info: &FftInfo) {
    todo!();
}

fn convolution_naive(a: &Vec<i64>, b: &Vec<i64>) -> Vec<i64> {
    todo!();
}

fn convolution_fft(mut a: Vec<i64>, mut b: Vec<i64>) -> Vec<i64> {
    todo!();
}

pub fn convolution(a: &[i64], b: &[i64]) -> Vec<i64> {
    todo!();
}

pub fn convolution_ll(a: &[i64], b: &[i64]) -> Vec<i64> {
    todo!();
}
