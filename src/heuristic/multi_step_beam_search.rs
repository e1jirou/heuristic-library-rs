#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

type Cost = i32;
type Hash = u64;
type CandidateIndex = u32;
type NodeIndex = u32;
type TurnIndex = u32;

// beam search setting
// capacity = 0 is OK.
#[derive(Debug, Clone)]
struct Config {
    max_turn: usize,
    beam_width: usize,
    nodes_capacity: usize,
}

// data for a state transition
// Try to minimize memory usage.
#[derive(Debug, Clone, Default)]
struct Action {
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
        todo!()
    }
}

// data for an expansion node
#[derive(Debug, Clone)]
struct Candidate {
    action: Action,
    evaluator: Evaluator,
    cost: Cost,
    hash: Hash,
    parent: NodeIndex,
}

// erasable max priority queue
#[derive(Debug, Clone)]
struct SegmentTree {
    n: usize,
    log: usize,
    size: usize,
    d: Vec<(Cost, CandidateIndex)>,
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
        self.d
            .resize(2 * self.size, (Cost::MIN, CandidateIndex::MAX));
    }

    fn build(&mut self, candidates: &Vec<Candidate>) {
        self.init(candidates.len());
        for i in 0..self.n {
            self.d[self.size + i] = (candidates[i].cost, i as CandidateIndex);
        }
        for i in (1..self.size).rev() {
            self.update(i);
        }
    }

    fn set(&mut self, p: usize, cost: Cost) {
        debug_assert!(p < self.n);
        let q = p + self.size;
        self.d[q] = (cost, p as CandidateIndex);
        for i in 1..=self.log {
            self.update(q >> i);
        }
    }

    fn top(&self) -> (Cost, CandidateIndex) {
        self.d[1]
    }

    fn update(&mut self, k: usize) {
        let (c0, p0) = self.d[2 * k];
        let (c1, p1) = self.d[2 * k + 1];
        self.d[k] = if c0 >= c1 { (c0, p0) } else { (c1, p1) };
    }
}

// select beam width candidates from better evaluation
// It remains only the best one for same hash candidates.
#[derive(Debug, Clone)]
struct Selector {
    beam_width: usize,
    candidates: Vec<Candidate>,
    hash_to_index: rustc_hash::FxHashMap<Hash, CandidateIndex>,
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
                let p = p as usize;
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
                    let p = self.segtree.top().1 as usize;
                    self.hash_to_index.remove(&self.candidates[p].hash);
                    self.hash_to_index.insert(hash, p as CandidateIndex);
                    self.candidates[p] = candidate;
                    self.segtree.set(p, cost);
                } else {
                    let p = self.candidates.len();
                    self.hash_to_index.insert(hash, p as CandidateIndex);
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
        self.candidates
            .iter()
            .min_by_key(|candidate| candidate.cost)
            .unwrap()
            .clone()
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
struct MultiSelectors {
    beam_width: usize,
    selectors: std::collections::VecDeque<Selector>,
    step_max: usize,
}

impl MultiSelectors {
    fn new(beam_width: usize) -> Self {
        Self {
            beam_width,
            selectors: std::collections::VecDeque::new(),
            step_max: 0,
        }
    }

    #[inline(always)]
    fn push_candidate(&mut self, candidate: Candidate, finished: bool, step: usize) -> bool {
        debug_assert!(step >= 1);
        while self.selectors.len() < step {
            self.selectors.push_back(Selector::new(self.beam_width));
        }
        if self.selectors[step - 1].push(candidate, finished) {
            if self.step_max < step {
                self.step_max = step;
            }
            true
        } else {
            false
        }
    }

    fn reset_step_max(&mut self) {
        self.step_max = 1;
    }

    fn pop_front(&mut self) -> Selector {
        debug_assert!(!self.selectors.is_empty());
        self.selectors.pop_front().unwrap()
    }

    fn push_back(&mut self, mut selector: Selector) {
        selector.clear();
        self.selectors.push_back(selector);
    }
}

#[derive(Debug, Clone)]
struct State {
    // TODO
}

// data updated in depth first search
impl State {
    fn new(seed: u64) -> Self {
        todo!()
    }

    // return the initial value of Action, Evaluator and Hash
    fn get_initial_data(&self) -> (Action, Evaluator, Hash) {
        todo!()
    }

    // add candidates to selector
    // argument
    // - evaluator: current evaluator
    // - hash: current hash
    // - parent: current node ID (parent node ID for next state)
    fn expand(
        &mut self,
        action: &Action,
        evaluator: &Evaluator,
        mut hash: Hash,
        parent: NodeIndex,
        selector: &mut MultiSelectors,
    ) {
        todo!()
    }

    fn move_forward(&mut self, action: &Action) {
        todo!()
    }

    fn move_backward(&mut self, action: &Action) {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct ObjectPool<T> {
    data: Vec<T>,
    garbage: Vec<usize>,
}

impl<T: Default> ObjectPool<T> {
    fn with_capacity(capacity: usize) -> ObjectPool<T> {
        ObjectPool {
            data: Vec::with_capacity(capacity),
            garbage: Vec::new(),
        }
    }

    // push `item`, then return the index
    fn push(&mut self, item: T) -> usize {
        if let Some(i) = self.garbage.pop() {
            self.data[i] = item;
            i
        } else {
            self.data.push(item);
            self.data.len() - 1
        }
    }

    fn pull(&mut self) -> usize {
        if let Some(i) = self.garbage.pop() {
            i
        } else {
            self.data.push(T::default());
            self.data.len() - 1
        }
    }

    // remove the item at position `index`
    fn remove(&mut self, index: usize) {
        self.garbage.push(index);
    }

    fn clear(&mut self) {
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
    parent: NodeIndex,
    child: NodeIndex,
    left: NodeIndex,
    right: NodeIndex,
    active: bool,
    remove_check_turn: TurnIndex,
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
            active: true,
            remove_check_turn: 0,
        }
    }

    fn new(candidate: &Candidate, right: NodeIndex) -> Self {
        Self {
            action: candidate.action.clone(),
            evaluator: candidate.evaluator.clone(),
            hash: candidate.hash,
            parent: candidate.parent,
            child: !0,
            left: !0,
            right,
            active: true,
            remove_check_turn: 0,
        }
    }
}

// doubly chained tree
#[derive(Debug, Clone)]
struct Tree {
    state: State,
    nodes: ObjectPool<Node>,
    root: NodeIndex,
    remove_nodes: std::collections::VecDeque<Vec<NodeIndex>>,
}

impl Tree {
    fn new(state: State, config: &Config) -> Self {
        let (action, evaluator, hash) = state.get_initial_data();
        let mut nodes = ObjectPool::with_capacity(config.nodes_capacity);
        let root = nodes.push(Node::root(action, evaluator, hash)) as NodeIndex;
        let remove_nodes = std::collections::VecDeque::new();
        Self {
            state,
            nodes,
            root,
            remove_nodes,
        }
    }

    // add candidates while dfs
    fn dfs(&mut self, multi_selectors: &mut MultiSelectors, turn: TurnIndex) {
        self.remove_useless_nodes(turn);
        self.update_root(turn);

        let mut v = self.root;

        if !self.nodes[v as usize].active {
            // no active nodes
            return;
        }

        loop {
            v = self.move_to_leaf(v);

            multi_selectors.reset_step_max();
            self.state.expand(
                &self.nodes[v as usize].action,
                &self.nodes[v as usize].evaluator,
                self.nodes[v as usize].hash,
                v,
                multi_selectors,
            );
            while self.remove_nodes.len() < multi_selectors.step_max {
                self.remove_nodes.push_back(Vec::new());
            }
            self.remove_nodes[multi_selectors.step_max - 1].push(v);
            self.nodes[v as usize].remove_check_turn = turn + multi_selectors.step_max as TurnIndex;

            v = self.move_to_ancestor(v);
            if v == self.root {
                break;
            }
        }
    }

    // get path from the root to the node `v`
    fn get_path(&self, mut v: NodeIndex) -> Vec<Action> {
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
    fn add_leaf(&mut self, candidate: &Candidate) -> NodeIndex {
        let parent = candidate.parent;
        let sibling = self.nodes[parent as usize].child;
        let v = self.nodes.push(Node::new(candidate, sibling)) as NodeIndex;

        self.nodes[parent as usize].child = v;

        if sibling != !0 {
            self.nodes[sibling as usize].left = v;
        }
        // activate ancestors
        let mut u = parent;
        while !self.nodes[u as usize].active {
            self.nodes[u as usize].active = true;
            if u == self.root {
                break;
            }
            u = self.nodes[u as usize].parent;
        }
        v
    }

    // move to the leftist node in the subtree rooted at `v`
    fn move_to_leaf(&mut self, mut v: NodeIndex) -> NodeIndex {
        debug_assert!(self.nodes[v as usize].active);
        let mut child = self.nodes[v as usize].child;
        while child != !0 {
            // move to right while the node is inactive
            while !self.nodes[child as usize].active {
                child = self.nodes[child as usize].right;
                debug_assert_ne!(child, !0);
            }
            self.nodes[v as usize].active = false;
            v = child;
            self.state.move_forward(&self.nodes[child as usize].action);

            #[cfg(target_arch = "x86_64")]
            unsafe {
                // prefetch the right sibling
                let right = self.nodes[child as usize].right;
                if right != !0 {
                    let ptr = self.nodes.data.as_ptr().add(right as usize);
                    _mm_prefetch(ptr as *const i8, _MM_HINT_T0);
                }
            }
            child = self.nodes[child as usize].child;
        }
        debug_assert!(self.nodes[v as usize].active);
        self.nodes[v as usize].active = false;
        v
    }

    // move to the ancestor of `v` which has the right child
    fn move_to_ancestor(&mut self, mut v: NodeIndex) -> NodeIndex {
        while v != self.root {
            self.state.move_backward(&self.nodes[v as usize].action);

            // move to right while the node is inactive
            let mut u = self.nodes[v as usize].right;
            while u != !0 {
                if self.nodes[u as usize].active {
                    self.state.move_forward(&self.nodes[u as usize].action);
                    return u;
                }
                u = self.nodes[u as usize].right;
            }

            v = self.nodes[v as usize].parent;
        }
        self.root
    }

    // do not round trip the direct road
    fn update_root(&mut self, turn: TurnIndex) {
        let mut child = self.nodes[self.root as usize].child;
        while child != !0
            && self.nodes[child as usize].right == !0
            && self.nodes[self.root as usize].remove_check_turn <= turn
        {
            self.root = child;
            self.state.move_forward(&self.nodes[child as usize].action);
            child = self.nodes[child as usize].child;
        }
    }

    // remove useless nodes
    fn remove_useless_nodes(&mut self, turn: TurnIndex) {
        if self.remove_nodes.is_empty() {
            return;
        }
        let mut remove_nodes_front = self.remove_nodes.pop_front().unwrap();
        for &v in &remove_nodes_front {
            if self.nodes[v as usize].child == !0 {
                self.remove_leaf(v, turn);
            }
        }
        remove_nodes_front.clear();
        self.remove_nodes.push_back(remove_nodes_front);
    }

    // remove the node `v` and its ancestors while they have no child
    fn remove_leaf(&mut self, mut v: NodeIndex, turn: TurnIndex) {
        loop {
            if self.nodes[v as usize].remove_check_turn > turn {
                // v can have a child in the future
                return;
            }
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

fn beam_search(config: &Config, state: State) -> Option<Vec<Action>> {
    let mut tree = Tree::new(state, config);
    let mut multi_selectors = MultiSelectors::new(config.beam_width);

    for turn in 0..config.max_turn {
        // add candidates to selector
        tree.dfs(&mut multi_selectors, turn as TurnIndex);

        let selector = multi_selectors.pop_front();
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
            tree.add_leaf(candidate);
        }

        multi_selectors.push_back(selector);
    }

    unreachable!();
}
