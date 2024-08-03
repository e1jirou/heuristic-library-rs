// T: unsigned integer
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: num_traits::PrimInt + num_traits::WrappingMul,
{
    pub fn encode(&self, w: T) -> T {
        w * self.x + self.y
    }

    pub fn decode(v: T, w: T) -> Self {
        Self {
            x: v / w,
            y: v % w,
        }
    }

    pub fn valid(&self, h: T, w: T) -> bool {
        self.x < h && self.y < w
    }

    pub fn mul(&self, v: T) -> Self {
        Self {
            x: self.x.wrapping_mul(&v),
            y: self.y.wrapping_mul(&v),
        }
    }
}

pub const UP: Point<usize> = Point {
    x: !0,
    y: 0,
};

pub const DOWN: Point<usize> = Point {
    x: 1,
    y: 0,
};

pub const LEFT: Point<usize> = Point {
    x: 0,
    y: !0,
};

pub const RIGHT: Point<usize> = Point {
    x: 0,
    y: 1,
};

pub const DXDY: [Point<usize>; 4] = [
    UP,
    DOWN,
    LEFT,
    RIGHT,
];

impl<T> std::ops::AddAssign for Point<T>
where
    T: num_traits::WrappingAdd,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x.wrapping_add(&rhs.x);
        self.y = self.y.wrapping_add(&rhs.y);
    }
}

impl<T> std::ops::SubAssign for Point<T>
where
    T: num_traits::WrappingSub,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x.wrapping_sub(&rhs.x);
        self.y = self.y.wrapping_sub(&rhs.y);
    }
}

impl<T> std::ops::Neg for Point<T>
where
    T: num_traits::WrappingNeg,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: self.x.wrapping_neg(),
            y: self.y.wrapping_neg(),
        }
    }
}

impl<T> std::ops::Add for Point<T>
where
    T: num_traits::WrappingAdd,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.wrapping_add(&rhs.x),
            y: self.y.wrapping_add(&rhs.y),
        }
    }
}

impl<T> std::ops::Sub for Point<T>
where
    T: num_traits::WrappingSub,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.wrapping_sub(&rhs.x),
            y: self.y.wrapping_sub(&rhs.y),
        }
    }
}
