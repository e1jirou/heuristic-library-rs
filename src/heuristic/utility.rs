use itertools::Itertools;

pub fn print_slice<T: std::fmt::Display>(v: &[T], sep: &str) {
    if v.is_empty() {
        return;
    }
    use std::io::Write;
    let mut handle = std::io::BufWriter::new(std::io::stdout().lock());
    for i in 0..(v.len() - 1) {
        write!(handle, "{}{}", v[i], sep).unwrap();
    }
    write!(handle, "{}{}", *v.last().unwrap(), "\n").unwrap();
    handle.flush().unwrap();
}

pub trait ChangeMinMax {
    fn chmin(&mut self, x: Self) -> bool;
    fn chmax(&mut self, x: Self) -> bool;
}

impl<T: PartialOrd> ChangeMinMax for T {
    fn chmin(&mut self, x: T) -> bool {
        *self > x && {
            *self = x;
            true
        }
    }

    fn chmax(&mut self, x: T) -> bool {
        *self < x && {
            *self = x;
            true
        }
    }
}

pub fn argmin<T: Ord>(v: &[T]) -> usize {
    debug_assert!(!v.is_empty());
    v.iter().enumerate().min_by_key(|&(_, x)| x).unwrap().0
}

pub fn argmax<T: Ord>(v: &[T]) -> usize {
    debug_assert!(!v.is_empty());
    v.iter().enumerate().max_by_key(|&(_, x)| x).unwrap().0
}

pub fn argsort<T: Ord>(v: &[T]) -> Vec<usize> {
    (0..v.len()).sorted_by_key(|&i| &v[i]).collect()
}

pub fn rperm(p: &[usize]) -> Vec<usize> {
    let n = p.len();
    let mut ret = vec![usize::MAX; n];
    for (i, &x) in p.iter().enumerate() {
        debug_assert!(x < n);
        debug_assert!(ret[x] == usize::MAX);
        ret[x] = i;
    }
    ret
}

pub fn get_time_sec() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        ms - STIME
    }
}

// NOTE: This is technically UB.
// The allocation is 512-byte aligned, but deallocation happens with a smaller alignment.
// We rely on the behavior of the system allocator and intentionally leak the memory
// using `mem::forget`, since this is short-lived competitive programming code.
pub fn aligned_alloc_512<T>(len: usize) -> Vec<T> {
    debug_assert_eq!(512 % std::mem::size_of::<T>(), 0);
    let size = len * std::mem::size_of::<T>();
    let size_aligned = (size + 511) / 512 * 512;
    let capacity = size_aligned / std::mem::size_of::<T>();
    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(size_aligned, 512);
        let ptr = std::alloc::alloc(layout);
        Vec::from_raw_parts(ptr as *mut T, len, capacity)
    }
}

#[inline(always)]
pub fn encode2(a: usize, b: usize, bw: usize) -> usize {
    debug_assert!(b < (1 << bw));
    (a << bw) | b
}

#[inline(always)]
pub fn decode2(ab: usize, bw: usize) -> (usize, usize) {
    let a = ab >> bw;
    let b = ab & ((1 << bw) - 1);
    (a, b)
}

#[inline(always)]
pub fn encode3(a: usize, b: usize, c: usize, bw: usize, cw: usize) -> usize {
    debug_assert!(b < (1 << bw));
    debug_assert!(c < (1 << cw));
    (a << (bw + cw)) | (b << cw) | c
}

#[inline(always)]
pub fn decode3(abc: usize, bw: usize, cw: usize) -> (usize, usize, usize) {
    let a = abc >> (bw + cw);
    let b = (abc >> cw) & ((1 << bw) - 1);
    let c = abc & ((1 << cw) - 1);
    (a, b, c)
}

#[inline(always)]
pub fn encode4(a: usize, b: usize, c: usize, d: usize, bw: usize, cw: usize, dw: usize) -> usize {
    debug_assert!(b < (1 << bw));
    debug_assert!(c < (1 << cw));
    debug_assert!(d < (1 << dw));
    (a << (bw + cw + dw)) | (b << (cw + dw)) | (c << dw) | d
}

#[inline(always)]
pub fn decode4(abcd: usize, bw: usize, cw: usize, dw: usize) -> (usize, usize, usize, usize) {
    let a = abcd >> (bw + cw + dw);
    let b = (abcd >> (cw + dw)) & ((1 << bw) - 1);
    let c = (abcd >> dw) & ((1 << cw) - 1);
    let d = abcd & ((1 << dw) - 1);
    (a, b, c, d)
}
