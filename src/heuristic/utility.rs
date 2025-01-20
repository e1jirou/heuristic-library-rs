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
