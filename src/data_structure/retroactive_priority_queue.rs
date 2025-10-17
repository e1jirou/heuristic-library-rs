#[derive(Debug, Clone)]
struct Node<T: Clone> {
    ssum: i32, // subtree sum of +1 (push) or -1 (pop) or 0 (no-op)
    smin: i32, // minimum of prefix sum of +1 (push) or -1 (pop) or 0 (no-op)
    qmax: T,   // maximum value in Q_now
    dmin: T,   // minimum value not in Q_now
}

#[derive(Debug, Clone)]
pub struct ReturnType<T> {
    pub insert: Vec<T>,
    pub erase: Vec<T>,
}

/// max priority queue with partial retroactivity
#[derive(Debug, Clone)]
pub struct PartiallyRetroactivePriorityQueue<T: Copy + Ord> {
    n: usize,           // number of operations to priority queue
    m: usize,           // number of leaves in segment tree
    tree: Vec<Node<T>>, // segment tree
}

impl<T: Copy + Ord> PartiallyRetroactivePriorityQueue<T> {
    pub fn new(n: usize, tmin: T, tmax: T) -> Self {
        debug_assert!(tmin < tmax);
        let m = (n + 1).next_power_of_two();
        let tree = vec![
            Node {
                ssum: 0,
                smin: 0,
                qmax: tmin,
                dmin: tmax,
            };
            2 * m
        ];
        Self { n, m, tree }
    }

    pub fn set_push(&mut self, mut i: usize, x: T) -> ReturnType<T> {
        debug_assert!(i < self.n);
        debug_assert!(self.tree[0].qmax < x && x < self.tree[0].dmin);
        let mut ret = self.set_no_op(i);
        i += self.m + 1;
        self.tree[i].dmin = x;
        self.update_d(i);
        self.incremental_update(i, &mut ret);
        ret
    }

    pub fn set_pop(&mut self, mut i: usize) -> ReturnType<T> {
        debug_assert!(i < self.n);
        let mut ret = self.set_no_op(i);
        i += self.m + 1;
        self.decremental_update(i, &mut ret);
        ret
    }

    pub fn set_no_op(&mut self, mut i: usize) -> ReturnType<T> {
        debug_assert!(i < self.n);
        let mut ret = ReturnType {
            insert: Vec::new(),
            erase: Vec::new(),
        };
        i += self.m + 1;
        if self.tree[i].ssum == -1 {
            self.incremental_update(i, &mut ret);
        } else if self.tree[0].qmax < self.tree[i].qmax {
            ret.erase.push(self.tree[i].qmax);
            self.tree[i].qmax = self.tree[0].qmax;
            self.update_q(i);
        } else if self.tree[i].dmin < self.tree[0].dmin {
            self.tree[i].dmin = self.tree[0].dmin;
            self.update_d(i);
            self.decremental_update(i, &mut ret);
        }
        ret
    }

    fn update_s(&mut self, mut i: usize) {
        // bottom up
        while i >= 2 {
            i >>= 1;
            self.tree[i].ssum = self.tree[i << 1].ssum + self.tree[i << 1 | 1].ssum;
            self.tree[i].smin = self.tree[i << 1]
                .smin
                .min(self.tree[i << 1].ssum + self.tree[i << 1 | 1].smin);
        }
    }

    fn update_q(&mut self, mut i: usize) {
        // bottom up
        while i >= 2 {
            i >>= 1;
            self.tree[i].qmax = self.tree[i << 1].qmax.max(self.tree[i << 1 | 1].qmax);
        }
    }

    fn update_d(&mut self, mut i: usize) {
        // bottom up
        while i >= 2 {
            i >>= 1;
            self.tree[i].dmin = self.tree[i << 1].dmin.min(self.tree[i << 1 | 1].dmin);
        }
    }

    fn incremental_update(&mut self, i: usize, ret: &mut ReturnType<T>) {
        self.tree[i].ssum += 1;
        self.update_s(i);

        // top down
        let mut s = self.tree[1].ssum - 1;
        let mut k = 1;
        while k < self.m {
            k <<= 1;
            if self.tree[k].ssum + self.tree[k | 1].smin == s {
                s -= self.tree[k].ssum;
                k |= 1;
            }
        }
        if k == self.m {
            return;
        }
        // bottom up
        let mut c = 0;
        let mut r = self.tree.len();
        while k < r {
            if k & 1 != 0 {
                if self.tree[k].dmin < self.tree[c].dmin {
                    c = k;
                }
                k += 1;
            }
            k >>= 1;
            r >>= 1;
        }
        // top down
        debug_assert_ne!(c, 0);
        while c < self.m {
            c <<= 1;
            if self.tree[c | 1].dmin < self.tree[c].dmin {
                c |= 1;
            }
        }
        ret.insert.push(self.tree[c].dmin);
        self.tree[c].ssum = 0;
        self.tree[c].qmax = self.tree[c].dmin;
        self.tree[c].dmin = self.tree[0].dmin;
        self.update_s(c);
        self.update_q(c);
        self.update_d(c);
    }

    fn decremental_update(&mut self, i: usize, ret: &mut ReturnType<T>) {
        self.tree[i].ssum -= 1;
        self.update_s(i);

        // top down
        let mut s = self.tree[1].ssum;
        let mut k = 1;
        while k < self.m {
            k <<= 1;
            if s != self.tree[k].smin {
                s -= self.tree[k].ssum;
                k |= 1;
            }
        }
        // bottom up
        let mut c = 0;
        while k > 0 {
            if k & 1 != 0 {
                k -= 1;
                if self.tree[c].qmax < self.tree[k].qmax {
                    c = k;
                }
            }
            k >>= 1;
        }
        if c == 0 {
            return;
        }
        // top down
        while c < self.m {
            c <<= 1;
            if self.tree[c].qmax < self.tree[c | 1].qmax {
                c |= 1;
            }
        }
        ret.erase.push(self.tree[c].qmax);
        self.tree[c].ssum = 1;
        self.tree[c].dmin = self.tree[c].qmax;
        self.tree[c].qmax = self.tree[0].qmax;
        self.update_s(c);
        self.update_q(c);
        self.update_d(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rand::prelude::*;
    use rand_pcg::Pcg64Mcg;
    use std::collections::{BinaryHeap, HashMap};

    #[derive(Debug, Clone, Copy)]
    enum Op {
        Push(i32),
        Pop,
        NoOp,
    }

    #[test]
    fn test_retroactive_priority_queue_returns_multiple() {
        const NUM_TRIALS: usize = 1000;
        const N: usize = 100;
        const Q: usize = 10 * N;
        const XMAX: i32 = 1000;

        for trial in 0..NUM_TRIALS {
            let mut rng = Pcg64Mcg::seed_from_u64(trial as u64);
            let n = rng.random_range(1..=N);
            let mut prpq = PartiallyRetroactivePriorityQueue::new(n, i32::MIN, i32::MAX);
            let mut ops = vec![Op::NoOp; n];
            let mut hm = HashMap::new();
            for _ in 0..Q {
                let i = rng.random_range(0..n);
                let op_type = rng.random_range(0..3);
                let diff = match op_type {
                    0 => {
                        let x = rng.random_range(0..XMAX);
                        ops[i] = Op::Push(x);
                        prpq.set_push(i, x)
                    }
                    1 => {
                        ops[i] = Op::Pop;
                        prpq.set_pop(i)
                    }
                    _ => {
                        ops[i] = Op::NoOp;
                        prpq.set_no_op(i)
                    }
                };
                for &x in &diff.insert {
                    hm.entry(x).or_insert(0);
                    *hm.get_mut(&x).unwrap() += 1;
                }
                for &x in &diff.erase {
                    if !hm.contains_key(&x) {
                        panic!("Tried to erase a value not in the queue: {}", x);
                    }
                    let count = hm.get_mut(&x).unwrap();
                    if *count <= 0 {
                        panic!("Tried to erase a value with count <= 0: {}", x);
                    }
                    *count -= 1;
                    if *count == 0 {
                        hm.remove(&x);
                    }
                }
                // naive check
                let mut heap = BinaryHeap::new();
                for i in 0..n {
                    match ops[i] {
                        Op::Push(x) => {
                            heap.push(x);
                        }
                        Op::Pop => {
                            heap.pop();
                        }
                        Op::NoOp => {}
                    }
                }
                let expected = heap.into_iter().sorted().collect::<Vec<_>>();
                let mut actual = vec![];
                for (&x, &count) in &hm {
                    for _ in 0..count {
                        actual.push(x);
                    }
                }
                actual.sort();
                assert_eq!(expected, actual, "Trial {} failed", trial);
            }
        }
    }
}
