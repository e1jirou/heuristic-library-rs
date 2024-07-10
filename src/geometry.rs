use num_complex::Complex;
use std::f64::consts::PI;

pub type Real = f64;
pub type Point = Complex<Real>;

pub const EPS: Real = 1e-9;

pub fn radian_to_degree(theta: f64) -> f64 {
    (180.0 / PI) * theta
}

pub fn degree_to_radian(degree: f64) -> f64 {
    (PI / 180.0) * degree
}

// almost equal
pub fn eq(a: Real, b: Real) -> bool {
    (b - a).abs() < EPS
}

// angle of b-a-c
pub fn angle(a: &Point, b: &Point, c: &Point) -> f64 {
    (c - a).arg() - (b - a).arg()
}

pub fn rot(p: &Point, theta: f64) -> Point {
    Point::from_polar(1.0, theta) * p
}

pub fn cross_product(a: &Point, b: &Point) -> Real {
    a.re * b.im - a.im * b.re
}

pub fn dot(a: &Point, b: &Point) -> Real {
    a.re * b.re + a.im * b.im
}

// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=CGL_1_C
// positional relationship between b and c from a
pub fn ccw(a: &Point, b: &Point, c: &Point) -> i32 {
    let ba = b - a;
    let ca = c - a;
    if cross_product(&ba, &ca) > EPS {
        1 // counter-clockwise
    } else if cross_product(&ba, &ca) < -EPS {
        -1 // clockwise
    } else if dot(&ba, &ca) < 0.0 {
        2 // online back (c-a-b)
    } else if ba.norm_sqr() < ca.norm_sqr() {
        -2 // online front (a-b-c)
    } else {
        0 // on segment (a-c-b)
    }
}

#[derive(Clone, PartialEq)]
pub struct Segment {
    a: Point,
    b: Point,
}

pub fn projection(s: &Segment, p: &Point) -> Point {
    let t = dot(&(p - s.a), &(s.b - s.a)) / (s.b - s.a).norm_sqr();
    s.a + (s.b - s.a).scale(t)
}

pub fn on_segment(s: &Segment, p: &Point) -> bool {
    ccw(&s.a, &s.b, p) == 0
}

pub fn intersect(s: &Segment, t: &Segment) -> bool {
    ccw(&s.a, &s.b, &t.a) * ccw(&s.a, &s.b, &t.b) <= 0 && ccw(&t.a, &t.b, &s.a) * ccw(&t.a, &t.b, &s.b) <= 0
}

pub fn calc_distance(s: &Segment, p: &Point) -> Real {
    let r = projection(s, p);
    if dot(&(s.a - r), &(s.b - r)) < 0.0 {
        (r - p).norm()
    } else {
        (s.a - p).norm().min((s.b - p).norm())
    }
}

pub fn calc_nearest_point(s: &Segment, p: &Point) -> Point {
    let r = projection(s, p);
    if dot(&(s.a - r), &(s.b - r)) < 0.0 {
        r
    } else if (s.a - p).norm_sqr() < (s.b - p).norm_sqr() {
        s.a
    } else {
        s.b
    }
}

pub fn cross_point(s: &Segment, t: &Segment) -> Point {
    let a = cross_product(&(s.b - s.a), &(t.b - t.a));
    let b = cross_product(&(s.b - s.a), &(s.b - t.a));
    if eq(a.abs(), 0.0) && eq(b.abs(), 0.0) {
        t.a
    } else {
        t.a + (b / a) * (t.b - t.a)
    }
}
