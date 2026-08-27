use std::process::exit;

use proconio::input;

#[allow(unused)]
const TIME_LIMIT_SEC: f64 = 1.95;

fn main() {
    get_time_sec();

    #[allow(unused)]
    let input = read_input();

    exit(0);
}

#[derive(Debug, Clone)]
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

// The following is a library published at https://github.com/e1jirou/heuristic-library-rs.

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
