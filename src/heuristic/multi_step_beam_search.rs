type Cost = i32;
type Hash = u64;
type Idx = u32;

// beam search setting
// capacity = 0 is OK.
#[derive(Debug, Clone)]
pub struct Config {
    max_turn: usize,
    beam_width: usize,
    nodes_capacity: usize,
}

// data for a state transition
// Try to minimize memory usage.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Action {
    // TODO
}

// data for evaluation
// Try to minimize memory usage.
#[derive(Debug, Clone, Default)]
struct Evaluator {
    // TODO
}

impl Evaluator {
    // the lower, the better
    fn evaluate(&self) -> Cost {
        todo!();
    }
}

// data for an expansion node
#[derive(Debug, Clone)]
struct Candidate {
    action: Action,
    evaluator: Evaluator,
    cost: Cost,
    hash: Hash,
    parent: Idx,
}

// erasable max priority queue
#[derive(Debug, Clone)]
struct SegmentTree {
    n: usize,
    log: usize,
    size: usize,
    d: Vec<(Cost, usize)>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut ret = Self {
            n: 0,
            log: 0,
            size: 0,
            d: Vec::new(),
        };
        ret.init(n);
        ret
    }

    fn init(&mut self, n: usize) {
        self.n = n;
        self.size = n.next_power_of_two();
        self.log = self.size.trailing_zeros() as usize;
        self.d.clear();
        self.d.resize(2 * self.size, (Cost::MIN, usize::MAX));
    }

    fn build(&mut self, candidates: &Vec<Candidate>) {
        self.init(candidates.len());
        for i in 0..self.n {
            self.d[self.size + i] = (candidates[i].cost, i);
        }
        for i in (1..self.size).rev() {
            self.update(i);
        }
    }

    fn set(&mut self, p: usize, cost: Cost) {
        debug_assert!(p < self.n);
        let q = p + self.size;
        self.d[q] = (cost, p);
        for i in 1..=self.log {
            self.update(q >> i);
        }
    }

    fn top(&self) -> (Cost, usize) {
        self.d[1]
    }

    fn update(&mut self, k: usize) {
        let (c0, p0) = self.d[2 * k];
        let (c1, p1) = self.d[2 * k + 1];
        self.d[k] = if c0 >= c1 {
            (c0, p0)
        } else {
            (c1, p1)
        };
    }
}

// select beam width candidates from better evaluation
// It remains only the best one for same hash candidates.
#[derive(Debug, Clone)]
struct Selector {
    beam_width: usize,
    candidates: Vec<Candidate>,
    hash_to_index: rustc_hash::FxHashMap<Hash,usize>,
    segtree: SegmentTree,
    finished_candidate: Option<Candidate>,
}

impl Selector {
    fn new(beam_width: usize) -> Self {
        Self {
            beam_width,
            candidates: Vec::with_capacity(beam_width),
            hash_to_index: rustc_hash::FxHashMap::default(),
            segtree: SegmentTree::new(beam_width),
            finished_candidate: None,
        }
    }

    // add a candidate
    // finished = true iff it can get feasible solution through the candidate in the turn minimizing problem.
    // It returns true iff it provisionally accept the candidate.
    #[inline(always)]
    fn push(&mut self, candidate: Candidate, finished: bool) -> bool {
        let cost = candidate.cost;
        if finished {
            match &self.finished_candidate {
                Some(best_candidate) => {
                    if cost < best_candidate.cost {
                        self.finished_candidate = Some(candidate);
                        return true;
                    } else {
                        return false;
                    }
                }
                None => {
                    self.finished_candidate = Some(candidate);
                    return true;
                }
            }
        }
        if self.built_segtree() && cost >= self.segtree.top().0 {
            return false;
        }
        let hash = candidate.hash;
        match self.hash_to_index.get(&hash) {
            Some(&p) => {
                // contains a candidate with same hash
                debug_assert_eq!(self.candidates[p].hash, hash);
                if cost < self.candidates[p].cost {
                    self.candidates[p] = candidate;
                    if self.built_segtree() {
                        self.segtree.set(p, cost);
                    }
                    true
                } else {
                    false
                }
            }
            None => {
                // does not contain a candidate with same hash
                if self.built_segtree() {
                    let p = self.segtree.top().1;
                    self.hash_to_index.remove(&self.candidates[p].hash);
                    self.hash_to_index.insert(hash, p);
                    self.candidates[p] = candidate;
                    self.segtree.set(p, cost);
                } else {
                    let p = self.candidates.len();
                    self.hash_to_index.insert(hash, p);
                    self.candidates.push(candidate);
                    if self.built_segtree() {
                        // candidates become full
                        self.segtree.build(&self.candidates);
                    }
                }
                true
            }
        }
    }

    fn get_best_candidate(&self) -> Candidate {
        self.candidates.iter().min_by_key(|candidate| candidate.cost).unwrap().clone()
    }

    fn clear(&mut self) {
        self.candidates.clear();
        self.hash_to_index.clear();
        self.finished_candidate = None;
    }

    fn built_segtree(&self) -> bool {
        self.candidates.len() == self.beam_width
    }
}

#[derive(Debug, Clone)]
pub struct State {
    // TODO
}

// data updated in depth first search
impl State {
    pub fn new() -> Self {
        todo!();
    }

    // return the initial value of Action, Evaluator and Hash
    fn get_initial_data(&self) -> (Action, Evaluator, Hash) {
        todo!();
    }

    // add candidates to selector
    // argument
    // - evaluator: current evaluator
    // - hash: current hash
    // - parent: current node ID (parent node ID for next state)
    fn expand(&self, evaluator: &Evaluator, mut hash: Hash, parent: Idx, selector: &mut Selector) {
        todo!();
    }

    fn move_forward(&mut self, action: &Action) {
        todo!();
    }

    fn move_backward(&mut self, action: &Action) {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct ObjectPool<T> {
    data: Vec<T>,
    garbage: Vec<usize>,
}

impl<T: Default> ObjectPool<T> {
    pub fn with_capacity(capacity: usize) -> ObjectPool<T> {
        ObjectPool {
            data: Vec::with_capacity(capacity),
            garbage: Vec::new(),
        }
    }

    // push `item`, then return the index
    pub fn push(&mut self, item: T) -> usize {
        if let Some(i) = self.garbage.pop() {
            self.data[i] = item;
            i
        } else {
            self.data.push(item);
            self.data.len() - 1
        }
    }

    pub fn pull(&mut self) -> usize {
        if let Some(i) = self.garbage.pop() {
            i
        } else {
            self.data.push(T::default());
            self.data.len() - 1
        }
    }

    // remove the item at position `index`
    pub fn remove(&mut self, index: usize) {
        self.garbage.push(index);
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.garbage.clear();
    }
}

impl<T> std::ops::Index<usize> for ObjectPool<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> std::ops::IndexMut<usize> for ObjectPool<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// node of doubly chained tree
#[derive(Debug, Clone, Default)]
struct Node {
    action: Action,
    evaluator: Evaluator,
    hash: Hash,
    parent: Idx,
    child: Idx,
    left: Idx,
    right: Idx,
}

impl Node {
    fn root(action: Action, evaluator: Evaluator, hash: Hash) -> Self {
        Self {
            action,
            evaluator,
            hash,
            parent: !0,
            child: !0,
            left: !0,
            right: !0,
        }
    }

    fn new(candidate: &Candidate, right: Idx) -> Self {
        Self {
            action: candidate.action.clone(),
            evaluator: candidate.evaluator.clone(),
            hash: candidate.hash,
            parent: candidate.parent,
            child: !0,
            left: !0,
            right,
        }
    }
}

// doubly chained tree
#[derive(Debug, Clone)]
struct Tree {
    state: State,
    nodes: ObjectPool<Node>,
    root: Idx,
}

impl Tree {
    fn new(state: State, config: &Config) -> Self {
        let (action, evaluator, hash) = state.get_initial_data();
        let mut nodes = ObjectPool::with_capacity(config.nodes_capacity);
        let root = nodes.push(Node::root(action, evaluator, hash)) as Idx;
        Self {
            state,
            nodes,
            root,
        }
    }

    // add candidates while dfs
    fn dfs(&mut self, selector: &mut Selector) {
        self.update_root();

        let mut v = self.root;
        loop {
            v = self.move_to_leaf(v);
            self.state.expand(&self.nodes[v as usize].evaluator, self.nodes[v as usize].hash, v, selector);
            v = self.move_to_ancestor(v);
            if v == self.root {
                break;
            }
            v = self.move_to_right(v);
        }
    }

    // get path from the root to the node `v`
    fn get_path(&self, mut v: Idx) -> Vec<Action> {
        // dbg!(self.nodes.data.capacity());

        let mut path = Vec::new();
        while self.nodes[v as usize].parent != !0 {
            path.push(self.nodes[v as usize].action.clone());
            v = self.nodes[v as usize].parent;
        }
        path.reverse();
        path
    }

    // add new node
    fn add_leaf(&mut self, candidate: &Candidate) -> Idx {
        let parent = candidate.parent;
        let sibling = self.nodes[parent as usize].child;
        let v = self.nodes.push(Node::new(candidate, sibling)) as Idx;

        self.nodes[parent as usize].child = v;

        if sibling != !0 {
            self.nodes[sibling as usize].left = v;
        }
        v
    }

    // remove the node `v` and its ancestors if they have no child
    fn remove_if_leaf(&mut self, v: Idx) {
        if self.nodes[v as usize].child == !0 {
            self.remove_leaf(v);
        }
    }

    // do not round trip the direct road
    fn update_root(&mut self) {
        let mut child = self.nodes[self.root as usize].child;
        while child != !0 && self.nodes[child as usize].right == !0 {
            self.root = child;
            self.state.move_forward(&self.nodes[child as usize].action);
            child = self.nodes[child as usize].child;
        }
    }

    // move to the leftist node in the subtree rooted at `v`
    fn move_to_leaf(&mut self, mut v: Idx) -> Idx {
        let mut child = self.nodes[v as usize].child;
        while child != !0 {
            v = child;
            self.state.move_forward(&self.nodes[child as usize].action);
            child = self.nodes[child as usize].child;
        }
        v
    }

    // move to the ancestor of `v` which has the right child
    fn move_to_ancestor(&mut self, mut v: Idx) -> Idx {
        while v != self.root && self.nodes[v as usize].right == !0 {
            self.state.move_backward(&self.nodes[v as usize].action);
            v = self.nodes[v as usize].parent;
        }
        v
    }

    // move to the right sibling of `v`
    fn move_to_right(&mut self, v: Idx) -> Idx {
        self.state.move_backward(&self.nodes[v as usize].action);
        let v = self.nodes[v as usize].right;
        self.state.move_forward(&self.nodes[v as usize].action);
        v
    }

    // remove the node `v` and its ancestors while they have no child
    fn remove_leaf(&mut self, mut v: Idx) {
        loop {
            let left = self.nodes[v as usize].left;
            let right = self.nodes[v as usize].right;
            if left == !0 {
                let parent = self.nodes[v as usize].parent;
                debug_assert_ne!(parent, !0, "root node cannot be removed");
                self.nodes.remove(v as usize);
                self.nodes[parent as usize].child = right;
                if right != !0 {
                    self.nodes[right as usize].left = !0;
                    break;
                }
                v = parent;
            } else {
                self.nodes.remove(v as usize);
                self.nodes[left as usize].right = right;
                if right != !0 {
                    self.nodes[right as usize].left = left;
                }
                break;
            }
        }
    }
}

pub fn beam_search(config: &Config, state: State) -> Option<Vec<Action>> {
    let mut tree = Tree::new(state, config);
    let mut curr_nodes = Vec::with_capacity(config.beam_width);
    let mut next_nodes = Vec::with_capacity(config.beam_width);
    let mut selector = Selector::new(config.beam_width);

    for turn in 0..config.max_turn {
        // add candidates to selector
        tree.dfs(&mut selector);

        if let Some(candidate) = &selector.finished_candidate {
            // find the feasible solution in turn minimizing problem
            let mut ret = tree.get_path(candidate.parent);
            ret.push(candidate.action.clone());
            return Some(ret);
        }
        if turn == config.max_turn - 1 {
            // the last turn
            let best_candidate = selector.get_best_candidate();
            let mut ret = tree.get_path(best_candidate.parent);
            ret.push(best_candidate.action);
            return Some(ret);
        }
        // add new nodes
        for candidate in &selector.candidates {
            next_nodes.push(tree.add_leaf(candidate));
        }
        if next_nodes.is_empty() {
            // cannot find the feasible solution
            return None;
        }
        // remove useless nodes
        for &v in &curr_nodes {
            tree.remove_if_leaf(v);
        }
        // double buffering
        std::mem::swap(&mut curr_nodes, &mut next_nodes);
        next_nodes.clear();

        selector.clear();
    }

    unreachable!();
}
