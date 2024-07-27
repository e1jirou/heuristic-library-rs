use proconio::input;
use itertools::Itertools;

#[allow(unused)]
const TIME_LIMIT_SEC: f64 = 1.95;

fn main() {
    get_time_sec();

    #[allow(unused)]
    let input = read_input();

    std::process::exit(0);
}

struct Input {
    // TODO
}

fn read_input() -> Input {
    input! {
        // TODO
    }
    Input {
        // TODO
    }
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
    for i in 0..n {
        debug_assert!(p[i] < n);
        debug_assert!(ret[p[i]] == usize::MAX);
        ret[p[i]] = i;
    }
    ret
}

pub fn bit_set<T: num_traits::PrimInt + std::ops::BitOrAssign>(x: &mut T, p: usize) -> bool {
    let y = T::one() << p;
    if (*x & y) == T::zero() {
        *x |= y;
        true
    } else {
        false
    }
}

pub fn bit_get<T: num_traits::PrimInt>(x: T, p: usize) -> bool {
    (x & (T::one() << p)) != T::zero()
}

pub fn pop_one<T: num_traits::PrimInt + std::ops::BitXorAssign>(x: &mut T) -> Option<u32> {
    if *x == T::zero() {
        return None;
    }
    let ret = x.trailing_zeros();
    *x ^= T::one() << ret as usize;
    Some(ret)
}

pub fn log_sum_pow(base: f64, v: &[f64]) -> f64 {
    let max_v = v.iter().max_by(|&&x, &y| x.partial_cmp(y).unwrap()).unwrap();
    v.iter().map(|&x| base.powf(x - max_v)).sum::<f64>().log(base) + max_v
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
