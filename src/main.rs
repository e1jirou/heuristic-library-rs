use proconio::input;

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
