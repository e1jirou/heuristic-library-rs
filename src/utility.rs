use fixedbitset::FixedBitSet;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use num_complex::Complex;
use std::collections::BinaryHeap;
use std::f64::consts::PI;
use std::ops::{Index, IndexMut};

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

#[allow(unused)]
pub struct IndexSet {
    data: Vec<usize>,
    pos: Vec<usize>,
}

impl IndexSet {
    #[allow(unused)]
    pub fn new(n: usize) -> Self {
        IndexSet {
            data: Vec::with_capacity(n),
            pos: vec![usize::MAX; n],
        }
    }

    #[allow(unused)]
    pub fn push(&mut self, x: usize) -> bool {
        if self.pos[x] == usize::MAX {
            self.pos[x] = self.data.len();
            self.data.push(x);
            true
        } else {
            false
        }
    }

    #[allow(unused)]
    pub fn pop(&mut self, x: usize) -> bool {
        let i = self.pos[x];
        if i == usize::MAX {
            return false;
        }
        let y = *self.data.last().unwrap();
        self.data[i] = y;
        self.data.pop();
        self.pos[y] = i;
        self.pos[x] = usize::MAX;
        true
    }

    #[allow(unused)]
    pub fn contains(&self, x: usize) -> bool {
        self.pos[x] != usize::MAX
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        for x in self.data.iter() {
            self.pos[*x] = usize::MAX;
        }
        self.data.clear();
    }
}

const SA_TIME_COUNTS: usize = 1 << 4;
const SA_RANDOM_STEPS: usize = 1 << 12;

#[allow(unused)]
pub enum SchedulerType {
    Exponential,
    Linear,
}

pub struct SimulatedAnnealingScheduler {
    schedule_type: SchedulerType,
    t_first: f64,
    t_last: f64,
    start_time_sec: f64,
    duration_sec: f64,
    time_counter: usize,
    temperature: f64,
    random_index: usize,
    log2_random: Vec<f64>,
}

impl SimulatedAnnealingScheduler {
    #[allow(unused)]
    pub fn new(
        schedule_type: SchedulerType,
        t_first: f64,
        t_last: f64,
        time_limit_sec: f64,
    ) -> SimulatedAnnealingScheduler {
        debug_assert!(0.0 <= t_last && t_last <= t_first);

        let mut log2_random = vec![0.0; SA_RANDOM_STEPS];
        for i in 0..SA_RANDOM_STEPS {
            log2_random[i] = ((i + 1) as f64 / SA_RANDOM_STEPS as f64).log2();
        }
        let mut rng = Pcg64Mcg::seed_from_u64(0);
        log2_random.shuffle(&mut rng);
        SimulatedAnnealingScheduler {
            schedule_type,
            t_first,
            t_last,
            start_time_sec: get_time_sec(),
            duration_sec: time_limit_sec - get_time_sec(),
            time_counter: 0,
            temperature: t_first,
            random_index: 0,
            log2_random,
        }
    }

    #[allow(unused)]
    pub fn accept(&mut self, profit: f64) -> bool {
        profit >= 0.0 || profit > self.get_threshold()
    }

    pub fn get_threshold(&mut self) -> f64 {
        self.update_temperature();
        if self.random_index == SA_RANDOM_STEPS - 1 {
            self.random_index = 0;
        } else {
            self.random_index += 1;
        }
        self.temperature * self.log2_random[self.random_index]
    }

    fn update_temperature(&mut self) {
        if self.time_counter > 0 {
            self.time_counter -= 1;
            return;
        }
        self.time_counter = SA_TIME_COUNTS - 1;
        let progress = (get_time_sec() - self.start_time_sec) / self.duration_sec;
        self.temperature = match self.schedule_type {
            SchedulerType::Exponential => {
                self.t_first.powf(1.0 - progress) * self.t_last.powf(progress)
            }
            SchedulerType::Linear => self.t_first * (1.0 - progress) + self.t_last * progress,
        }
    }
}

#[allow(unused)]
pub struct ObjectPool<T> {
    data: Vec<T>,
    garbage: Vec<usize>,
}

impl<T: Default> ObjectPool<T> {
    #[allow(unused)]
    pub fn with_capacity(capacity: usize) -> ObjectPool<T> {
        ObjectPool {
            data: Vec::with_capacity(capacity),
            garbage: Vec::new(),
        }
    }

    #[allow(unused)]
    // push `item``, then return the index
    pub fn push(&mut self, item: T) -> usize {
        if let Some(i) = self.garbage.pop() {
            self.data[i] = item;
            i
        } else {
            self.data.push(item);
            self.data.len() - 1
        }
    }

    #[allow(unused)]
    pub fn pull(&mut self) -> usize {
        if let Some(i) = self.garbage.pop() {
            i
        } else {
            self.data.push(T::default());
            self.data.len() - 1
        }
    }

    #[allow(unused)]
    // remove the item at position `index`
    pub fn remove(&mut self, index: usize) {
        self.garbage.push(index);
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.garbage = (0..self.data.len()).rev().collect();
    }
}

impl<T> Index<usize> for ObjectPool<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for ObjectPool<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[allow(unused)]
pub struct ErasablePriorityQueue<T> {
    data: BinaryHeap<T>,
    lazy: BinaryHeap<T>,
}

impl<T: Ord> ErasablePriorityQueue<T> {
    #[allow(unused)]
    pub fn new() -> Self {
        ErasablePriorityQueue {
            data: BinaryHeap::new(),
            lazy: BinaryHeap::new(),
        }
    }

    #[allow(unused)]
    pub fn with_capacity(capacity: usize) -> Self {
        ErasablePriorityQueue {
            data: BinaryHeap::with_capacity(capacity),
            lazy: BinaryHeap::with_capacity(capacity),
        }
    }

    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[allow(unused)]
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    #[allow(unused)]
    pub fn erase(&mut self, item: T) {
        if *self.data.peek().unwrap() == item {
            self.data.pop();
            self.update();
        } else {
            self.lazy.push(item);
        }
    }

    #[allow(unused)]
    pub fn peek(&self) -> Option<&T> {
        self.data.peek()
    }

    #[allow(unused)]
    fn update(&mut self) {
        while !self.lazy.is_empty() && self.data.peek().unwrap() == self.lazy.peek().unwrap() {
            self.data.pop();
            self.lazy.pop();
        }
    }
}

pub struct LowLinkForUndirectedGraph {
    edges: Vec<Vec<usize>>,
    ord: Vec<usize>,
    low: Vec<usize>,
    is_articulation_point: Vec<bool>,
}

impl LowLinkForUndirectedGraph {
    #[allow(unused)]
    pub fn new(edges: Vec<Vec<usize>>) -> Self {
        let n = edges.len();
        let mut ret = LowLinkForUndirectedGraph {
            edges,
            ord: vec![usize::MAX; n],
            low: vec![usize::MAX; n],
            is_articulation_point: vec![false; n],
        };
        ret.low_link(0, 0, usize::MAX);
        ret
    }

    fn low_link(&mut self, v: usize, mut i: usize, parent: usize) -> usize {
        debug_assert_eq!(self.ord[v], usize::MAX);
        self.ord[v] = i;
        i += 1;
        let mut num_children = 0;
        for edges_v_id in 0..self.edges[v].len() {
            let u = self.edges[v][edges_v_id];
            if self.ord[u] == usize::MAX {
                // forward edge
                num_children += 1;
                i = self.low_link(u, i, v);
                self.low[v] = self.low[v].min(self.low[u]);
                if self.ord[v] > 0 && self.ord[v] <= self.low[u] {
                    self.is_articulation_point[v] = true;
                }
            } else {
                // backward edge
                if u != parent {
                    self.low[v] = self.low[v].min(self.ord[u]);
                }
            }
        }
        if self.ord[v] == 0 && num_children >= 2 {
            // root is an articulation point
            self.is_articulation_point[v] = true;
        }
        i
    }
}

// This function aids in making a conservative assessment of
// whether a vertex within a grid is an articulation point,
// considering only the eight surrounding vertices.
// 0 1 2
// 3 4 5
// 6 7 8
#[allow(unused)]
pub fn make_connected3x3() -> FixedBitSet {
    let mut edges = vec![Vec::new(); 9];
    for v in 0..9 {
        if v / 3 > 0 {
            edges[v].push(v - 3);
        }
        if v % 3 > 0 {
            edges[v].push(v - 1);
        }
        if v % 3 < 2 {
            edges[v].push(v + 1);
        }
        if v / 3 < 2{
            edges[v].push(v + 3);
        }
    }
    let mut connected3x3 = FixedBitSet::with_capacity(512);
    for s in 1..512 as usize {
        // Depth First Search
        let root = s.trailing_zeros();
        let mut visited = 1 << root;
        let mut todo: usize = 1 << root;
        while todo > 0 {
            let u = todo.trailing_zeros() as usize;
            todo ^= 1 << u;
            for v in edges[u].iter() {
                if (s & (1 << v)) > 0 && (visited & (1 << v)) == 0 {
                    visited |= 1 << v;
                    todo |= 1 << v;
                }
            }
        }
        if visited == s {
            connected3x3.set(s, true);
        }
    }
    connected3x3
}

// calculate the TSP optimal solution
// start from the last vertex (and end to the last vertex)
#[allow(unused)]
pub fn solve_tsp(cost_matrix: &Vec<Vec<i64>>, back_to_the_start_point: bool) -> Vec<usize> {
    if cost_matrix.len() == 1 {
        return vec![0];
    }
    let n = cost_matrix.len() - 1;
    let inf = i64::max_value() / 2;

    // dp
    let mut dp = vec![vec![inf; 1 << n]; n];
    for v in 0..n {
        dp[v][1 << v] = cost_matrix[n][v];
    }
    for s in 1..(1 << n) {
        for u in 0..n {
            if ((s >> u) & 1) == 0 {
                continue;
            }
            for v in 0..n {
                if ((s >> v) & 1) == 0 {
                    let cost = dp[u][s] + cost_matrix[u][v];
                    dp[v][s | (1 << v)].chmin(cost);
                }
            }
        }
    }
    let mut last = n;
    let mut cost = inf;
    let mut s = (1 << n) - 1;
    for v in 0..n {
        if back_to_the_start_point {
            if cost.chmin(dp[v][s] + cost_matrix[v][n]) {
                last = v;
            }
        } else {
            if cost.chmin(dp[v][s]) {
                last = v;
            }
        }
    }
    let mut ret = vec![n; n + 1 + back_to_the_start_point as usize];
    ret[n] = last;
    s ^= 1 << last;
    for i in (1..n).rev() {
        for v in 0..n {
            if (s & (1 << v)) > 0 && dp[v][s] + cost_matrix[v][last] == dp[last][s | (1 << last)] {
                ret[i] = v;
                s ^= 1 << v;
                break;
            }
        }
        debug_assert_ne!(ret[i], n);
    }
    ret
}

// TODO: beam search

// geometry
pub type Real = f64;
pub type Point = Complex<Real>;

pub const EPS: Real = 1e-9;

#[allow(unused)]
pub fn radian_to_degree(theta: f64) -> f64 {
    (180.0 / PI) * theta
}

#[allow(unused)]
pub fn degree_to_radian(degree: f64) -> f64 {
    (PI / 180.0) * degree
}

// almost equal
#[allow(unused)]
pub fn eq(a: Real, b: Real) -> bool {
    (b - a).abs() < EPS
}

// angle of b-a-c
#[allow(unused)]
pub fn angle(a: &Point, b: &Point, c: &Point) -> f64 {
    (c - a).arg() - (b - a).arg()
}

#[allow(unused)]
pub fn rot(p: &Point, theta: f64) -> Point {
    Point::from_polar(1.0, theta) * p
}

#[allow(unused)]
pub fn cross_product(a: &Point, b: &Point) -> Real {
    a.re * b.im - a.im * b.re
}

#[allow(unused)]
pub fn dot(a: &Point, b: &Point) -> Real {
    a.re * b.re + a.im * b.im
}

// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=CGL_1_C
// positional relationship between b and c from a
#[allow(unused)]
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

#[allow(unused)]
pub fn projection(s: &Segment, p: &Point) -> Point {
    let t = dot(&(p - s.a), &(s.b - s.a)) / (s.b - s.a).norm_sqr();
    s.a + (s.b - s.a).scale(t)
}

#[allow(unused)]
pub fn on_segment(s: &Segment, p: &Point) -> bool {
    ccw(&s.a, &s.b, p) == 0
}

#[allow(unused)]
pub fn intersect(s: &Segment, t: &Segment) -> bool {
    ccw(&s.a, &s.b, &t.a) * ccw(&s.a, &s.b, &t.b) <= 0 && ccw(&t.a, &t.b, &s.a) * ccw(&t.a, &t.b, &s.b) <= 0
}

#[allow(unused)]
pub fn calc_distance(s: &Segment, p: &Point) -> f64 {
    let r = projection(s, p);
    if dot(&(s.a - r), &(s.b - r)) < 0.0 {
        (r - p).norm()
    } else {
        (s.a - p).norm().min((s.b - p).norm())
    }
}

#[allow(unused)]
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

#[allow(unused)]
pub fn cross_point(s: &Segment, t: &Segment) -> Point {
    let a = cross_product(&(s.b - s.a), &(t.b - t.a));
    let b = cross_product(&(s.b - s.a), &(s.b - t.a));
    if eq(a.abs(), 0.0) && eq(b.abs(), 0.0) {
        t.a
    } else {
        t.a + (b / a) * (t.b - t.a)
    }
}
