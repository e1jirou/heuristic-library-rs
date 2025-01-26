pub fn bitset_get(s: &[u64], i: usize) -> bool {
    debug_assert!(i < 64 * s.len());
    (s[i / 64] & (1 << (i & 63))) != 0
}

pub fn bitset_set(s: &mut [u64], i: usize, flag: bool) {
    debug_assert!(i < 64 * s.len());
    if flag {
        s[i / 64] |= 1 << (i & 63);
    } else {
        s[i / 64] &= !(1 << (i & 63));
    }
}

pub fn bitset_any(s: &[u64]) -> bool {
    s.iter().any(|&x| x != 0)
}

pub fn bitset_and_assign(s: &mut [u64], t: &[u64]) {
    for (x, &y) in s.iter_mut().zip(t.iter()) {
        *x &= y;
    }
}

pub fn bitset_or_assign(s: &mut [u64], t: &[u64]) {
    for (x, &y) in s.iter_mut().zip(t.iter()) {
        *x |= y;
    }
}

pub fn bitset_xor_assign(s: &mut [u64], t: &[u64]) {
    for (x, &y) in s.iter_mut().zip(t.iter()) {
        *x ^= y;
    }
}

pub fn bitset_clear(s: &mut [u64]) {
    s.fill(0);
}

pub fn bitset_count(s: &[u64]) -> u32 {
    let mut cnt = 0;
    for &x in s {
        cnt += x.count_ones();
    }
    cnt
}

pub fn bitset_range_set(s: &mut [u64], l: usize, r: usize, flag: bool) {
    debug_assert!(l <= r);
    debug_assert!(r <= 64 * s.len());
    if flag {
        if l / 64 == r / 64 {
            s[l / 64] |= (1 << (r & 63)) - (1 << (l & 63));
            return;
        }
        s[l / 64] |= (0 as u64).wrapping_sub(1 << (l & 63));
        for i in (l / 64 + 1)..(r / 64) {
            s[i] = u64::MAX;
        }
        s[r / 64] |= (1 << (r & 63)) - 1;
    } else {
        if l / 64 == r / 64 {
            s[l / 64] &= !((1 << (r & 63)) - (1 << (l & 63)));
            return;
        }
        s[l / 64] &= !((0 as u64).wrapping_sub(1 << (l & 63)));
        for i in (l / 64 + 1)..(r / 64) {
            s[i] = 0;
        }
        s[r / 64] &= !((1 << (r & 63)) - 1);
    }
}

pub fn bitset_get_ones(s: &[u64]) -> Vec<usize> {
    let mut ones = vec![];
    for i in 0..s.len() {
        let mut x = s[i];
        while x > 0 {
            let p = x.trailing_zeros() as usize;
            x ^= 1 << p;
            ones.push(64 * i + p);
        }
    }
    ones
}
