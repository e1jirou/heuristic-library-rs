fn safe_mod(mut x: i64, m: i64) -> i64 {
    debug_assert!(m >= 1);
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}

fn pow_mod(x: i64, mut n: i64, m: i64) -> i64 {
    debug_assert!(n >= 0);
    debug_assert!(m >= 1);
    debug_assert!((m as i32) as i64 == m);
    if m == 1 {
        return 0;
    }
    let mut r = 1;
    let mut y = safe_mod(x, m) as u64;
    while n != 0 {
        if (n & 1) > 0 {
            r = (r * y) % (m as u64);
        }
        y = (y * y) % (m as u64);
        n >>= 1;
    }
    r as i64
}

fn primitive_root(m: i64) -> i64 {
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
        for i in 0..cnt {
            if pow_mod(g, (m - 1) / divs[i], m) == 1 {
                ok = false;
                break;
            }
        }
        if ok {
            return g;
        }
        g += 1;
    }
}

struct FftInfo {
    g: i64,
    rank2: usize,
    root: Vec<i64>,
    iroot: Vec<i64>,
    rate2: Vec<i64>,
    irate2: Vec<i64>,
    rate3: Vec<i64>,
    irate3: Vec<i64>,
}

impl FftInfo {
    fn new(m: i64) -> Self {
        let g = primitive_root(m);
        let rank2 = 63 - ((m - 1) & (1 - m)).leading_zeros() as usize;

        let mut root = vec![0; rank2 + 1];
        root[rank2] = pow_mod(g, (m - 1) >> rank2, m);
        let mut iroot = vec![0; rank2 + 1];
        iroot[rank2] = pow_mod(root[rank2], m - 2, m);
        for i in (0..rank2).rev() {
            root[i] = root[i + 1] * root[i + 1] % m;
            iroot[i] = iroot[i + 1] * iroot[i + 1] % m;
        }
        let mut rate2 = vec![0; rank2 - 1];
        let mut irate2 = vec![0; rank2 - 1];
        let mut prod = 1;
        let mut iprod = 1;
        for i in 0..=(rank2 - 2) {
            rate2[i] = root[i + 2] * prod % m;
            irate2[i] = iroot[i + 2] * iprod % m;
            prod *= iroot[i + 2];
            prod %= m;
            iprod *= root[i + 2];
            iprod %= m;
        }
        let mut rate3 = vec![0; rank2 - 2];
        let mut irate3 = vec![0; rank2 - 2];
        let mut prod = 1;
        let mut iprod = 1;
        for i in 0..=(rank2 - 3) {
            rate3[i] = root[i + 3] * prod % m;
            irate3[i] = iroot[i + 3] * iprod % m;
            prod *= iroot[i + 3];
            prod %= m;
            iprod *= root[i + 3];
            iprod %= m;
        }
        FftInfo {
            g,
            rank2,
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
