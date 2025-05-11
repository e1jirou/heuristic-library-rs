// struct MaxMonoid;

// impl Monoid for MaxMonoid {
//     type S = i32;

//     fn e() -> Self::S {
//         Self::S::MIN
//     }

//     fn op(a: &Self::S, b: &Self::S) -> Self::S {
//         *a.max(b)
//     }
// }


pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

struct Node<M: Monoid> {
    index: usize,
    value: M::S,
    product: M::S,
    left: usize,
    right: usize,
}

pub struct DynamicSegmentTree<M: Monoid> {
    n: usize,
    nodes: Vec<Node<M>>,
}

impl<M: Monoid> DynamicSegmentTree<M> {
    pub fn new(n: usize) -> Self {
        let root_node = Node {
            index: 0,
            value: M::e(),
            product: M::e(),
            left: !0,
            right: !0,
        };
        Self {
            n,
            nodes: vec![root_node],
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }

    fn make_node(&mut self, index: usize, value: M::S) -> usize {
        let node = Node {
            index,
            value: value.clone(),
            product: value,
            left: !0,
            right: !0,
        };
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn set(&mut self, p: usize, x: M::S) {
        debug_assert!(p < self.n);
        self.set_internal(0, 0, self.n, p, x);
    }

    fn set_internal(&mut self, k: usize, a: usize, b: usize, mut p: usize, mut x: M::S) {
        if self.nodes[k].index == p {
            self.nodes[k].value = x;
            self.update(k);
            return;
        }
        let c = (a + b) >> 1;
        if p < c {
            if self.nodes[k].index < p {
                std::mem::swap(&mut self.nodes[k].index, &mut p);
                std::mem::swap(&mut self.nodes[k].value, &mut x);
            }
            if self.nodes[k].left == !0 {
                self.nodes[k].left = self.make_node(p, x);
            } else {
                self.set_internal(self.nodes[k].left, a, c, p, x);
            }
        } else {
            if self.nodes[k].index > p {
                std::mem::swap(&mut self.nodes[k].index, &mut p);
                std::mem::swap(&mut self.nodes[k].value, &mut x);
            }
            if self.nodes[k].right == !0 {
                self.nodes[k].right = self.make_node(p, x);
            } else {
                self.set_internal(self.nodes[k].right, c, b, p, x);
            }
        }
        self.update(k);
    }

    pub fn get(&self, p: usize) -> M::S {
        debug_assert!(p < self.n);
        self.get_internal(0, 0, self.n, p)
    }

    fn get_internal(&self, k: usize, a: usize, b: usize, p: usize) -> M::S {
        if k == !0 {
            return M::e();
        }
        let node = &self.nodes[k];
        if node.index == p {
            return node.value.clone();
        }
        let c = (a + b) >> 1;
        if p < c {
            self.get_internal(node.left, a, c, p)
        } else {
            self.get_internal(node.right, c, b, p)
        }
    }

    pub fn prod(&self, l: usize, r: usize) -> M::S {
        debug_assert!(l <= r && r <= self.n);
        self.prod_internal(0, 0, self.n, l, r)
    }

    fn prod_internal(&self, k: usize, a: usize, b: usize, l: usize, r: usize) -> M::S {
        if k == !0 || b <= l || r <= a {
            return M::e();
        }
        if l <= a && b <= r {
            return self.nodes[k].product.clone();
        }
        let c = (a + b) >> 1;
        let mut res = self.prod_internal(self.nodes[k].left, a, c, l, r);
        let node = &self.nodes[k];
        if l <= node.index && node.index < r {
            res = M::op(&res, &node.value);
        }
        M::op(&res, &self.prod_internal(node.right, c, b, l, r))
    }

    pub fn all_prod(&self) -> &M::S {
        &self.nodes[0].product
    }

    pub fn max_right<F>(&self, l: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        debug_assert!(l <= self.n);
        debug_assert!(f(&M::e()));
        self.max_right_internal(0, 0, self.n, l, &f, M::e())
    }

    fn max_right_internal<F>(&self, k: usize, a: usize, b: usize, l: usize, f: &F, mut product: M::S) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        if k == !0 || b <= l {
            return self.n;
        }
        let node = &self.nodes[k];
        if f(&M::op(&product, &node.product)) {
            return self.n;
        }
        let c = (a + b) >> 1;
        let res = self.max_right_internal(node.left, a, c, l, f, product.clone());
        if res != self.n {
            return res;
        }
        if l <= node.index {
            product = M::op(&product, &node.value);
            if !f(&product) {
                return node.index;
            }
        }
        self.max_right_internal(node.right, c, b, l, f, product)
    }

    pub fn min_left<F>(&self, r: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        debug_assert!(r <= self.n);
        debug_assert!(f(&M::e()));
        self.min_left_internal(0, 0, self.n, r, &f, M::e())
    }

    fn min_left_internal<F>(&self, k: usize, a: usize, b: usize, r: usize, f: &F, mut product: M::S) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        if k == !0 || r <= a {
            return 0;
        }
        let node = &self.nodes[k];
        if f(&M::op(&node.product, &product)) {
            return 0;
        }
        let c = (a + b) >> 1;
        let res = self.min_left_internal(node.right, c, b, r, f, product.clone());
        if res != 0 {
            return res;
        }
        if node.index < r {
            product = M::op(&node.value, &product);
            if !f(&product) {
                return node.index;
            }
        }
        self.min_left_internal(node.left, a, c, r, f, product)
    }

    fn update(&mut self, k: usize) {
        let node = &self.nodes[k];
        let mut product = node.value.clone();
        if node.left != !0 {
            product = M::op(&self.nodes[node.left].product, &product);
        }
        if node.right != !0 {
            product = M::op(&product, &self.nodes[node.right].product);
        }
        self.nodes[k].product = product;
    }
}
