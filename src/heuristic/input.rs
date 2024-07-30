use std::io::{self, Read};

pub struct Input {
    iter: std::str::SplitAsciiWhitespace<'static>,
    #[allow(unused)]
    buf: Box<str>,
}

impl Input {
    pub fn new() -> Self {
        let mut buf = String::new();
        io::stdin().lock().read_to_string(&mut buf).unwrap();
        let buf = buf.into_boxed_str();
        let iter = unsafe { std::mem::transmute::<_, &'static str>(&*buf) }.split_ascii_whitespace();
        Input { iter, buf }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
}

macro_rules! input {
    ($input:expr, $($r:tt)*) => {
        input_inner!{$input, $($r)*}
    };
}

macro_rules! input_inner {
    ($input:expr) => {};
    ($input:expr, ) => {};

    ($input:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($input, $t);
        input_inner!{$input $($r)*}
    };
}

macro_rules! read_value {
    ($input:expr, ( $($t:tt),* )) => {
        ( $(read_value!($input, $t)),* )
    };

    ($input:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($input, $t)).collect::<Vec<_>>()
    };

    ($input:expr, [ $t:tt ]) => {
        {
            let len = $input.next::<usize>();
            (0..len).map(|_| read_value!($input, $t)).collect::<Vec<_>>()
        }
    };

    ($input:expr, usize1) => {
        $input.next::<usize>() - 1
    };

    ($input:expr, $t:ty) => {
        $input.next::<$t>()
    };
}
