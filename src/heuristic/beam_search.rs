type Cost = i32;
type Hash = u64;

// beam search setting
// capacity = 0 is OK.
pub struct Config {
    max_turn: usize,
    beam_width: usize,
    tour_capacity: usize,
}

// data for a state transition
// Try to minimize memory usage.
#[derive(Clone, PartialEq, Eq)]
pub struct Action {
    // TODO
}

// data for evaluation
// Try to minimize memory usage.
#[derive(Clone)]
struct Evaluator {
    // TODO
}

impl Evaluator {
    // the lower, the better
    fn evaluate(&self) -> Cost {
        todo!();
    }
}

// data for an espansion node
#[derive(Clone)]
struct Candidate {
    action: Action,
    evaluator: Evaluator,
    cost: Cost,
    hash: Hash,
    parent: usize,
}

// erasable max priority queue
struct SegmentTree {
    n: usize,
    log: usize,
    size: usize,
    d: Vec<(Cost, usize)>,
}

impl SegmentTree {
    fn new(n: usize) -> SegmentTree {
        let size = n.next_power_of_two();
        let log = size.trailing_zeros() as usize;
        SegmentTree {
            n,
            log,
            size,
            d: vec![(Cost::MIN, usize::MAX); 2 * size],
        }
    }

    fn build(&mut self, candidates: &Vec<Candidate>) {
        debug_assert_eq!(candidates.len(), self.n);
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
struct Selector {
    beam_width: usize,
    candidates: Vec<Candidate>,
    hash_to_index: std::collections::HashMap<Hash,usize>,
    segtree: SegmentTree,
    finished_candidate: Option<Candidate>,
}

impl Selector {
    fn new(beam_width: usize) -> Selector {
        Selector {
            beam_width,
            candidates: Vec::with_capacity(beam_width),
            hash_to_index: std::collections::HashMap::with_capacity(beam_width),
            segtree: SegmentTree::new(beam_width),
            finished_candidate: None,
        }
    }

    // add a candidate
    // finished = true iff it can get feasible solution through the candidate in the turn minimizing problem.
    // It returns true iff it provisionally accept the candidate.
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

pub struct State {
    // TODO
}

// data updated in depth first search
impl State {
    pub fn new() -> State {
        todo!();
    }

    // return the initial value of Evaluator and Hash
    fn get_initial_data(&self) -> (Evaluator, Hash) {
        todo!();
    }

    // add candidates to selector
    // argument
    // - evaluator: current evaluator
    // - hash: current hash
    // - parent: current node ID (parent node ID for next state)
    fn expand(&self, evaluator: &Evaluator, mut hash: Hash, parent: usize, selector: &mut Selector) {
        todo!();
    }

    fn move_forward(&mut self, action: &Action) {
        todo!();
    }

    fn move_backward(&mut self, action: &Action) {
        todo!();
    }
}

#[derive(Clone, PartialEq, Eq)]
enum EdgeProperty {
    Leaf,
    Forward,
    Backward,
}

#[derive(Clone)]
struct Edge {
    property: EdgeProperty,
    action: Action,
}

struct Tree {
    state: State,
    curr_tour: Vec<Edge>,
    next_tour: Vec<Edge>,
    leaves: Vec<(Evaluator, Hash)>,
    buckets: Vec<Vec<(Action, Evaluator, Hash)>>,
    direct_road: Vec<Action>,
}

impl Tree {
    fn new(state: State, config: &Config) -> Tree {
        Tree {
            state,
            curr_tour: Vec::with_capacity(config.tour_capacity),
            next_tour: Vec::with_capacity(config.tour_capacity),
            leaves: Vec::with_capacity(config.beam_width as usize),
            buckets: vec![vec![]; config.beam_width as usize],
            direct_road: Vec::new(),
        }
    }

    // add candidates to selector while updating state
    fn dfs(&mut self, selector: &mut Selector) {
        if self.curr_tour.is_empty() {
            // the first turn
            let (evaluator, hash) = self.state.get_initial_data();
            self.state.expand(&evaluator, hash, 0, selector);
            return;
        }

        let mut leaf_id = 0;
        for edge in &self.curr_tour {
            match edge.property {
                EdgeProperty::Leaf => {
                    let (evaluator, hash) = &self.leaves[leaf_id];
                    self.state.move_forward(&edge.action);
                    self.state.expand(evaluator, *hash, leaf_id, selector);
                    self.state.move_backward(&edge.action);
                    leaf_id += 1;
                }
                EdgeProperty::Forward => self.state.move_forward(&edge.action),
                EdgeProperty::Backward => self.state.move_backward(&edge.action)
            }
        }
    }

    // add new nodes and remove useless nodes
    fn update(&mut self, candidates: &Vec<Candidate>) {
        self.leaves.clear();

        if self.curr_tour.is_empty() {
            // no branch
            self.curr_tour.clear();
            for candidate in candidates {
                self.curr_tour.push(Edge {
                    property: EdgeProperty::Leaf,
                    action: candidate.action.clone(),
                });
                self.leaves.push((candidate.evaluator.clone(), candidate.hash));
            }
            return;
        }

        // bucket sort
        for candidate in candidates {
            self.buckets[candidate.parent as usize].push(
                (candidate.action.clone(), candidate.evaluator.clone(), candidate.hash));
        }

        let mut curr_tour_id = 0;
        
        // do not repeat direct road
        let is_direct_road = |curr_tour_id: usize, curr_tour: &Vec<Edge>| {
            curr_tour[curr_tour_id].property == EdgeProperty::Forward &&
            curr_tour.last().unwrap().action == curr_tour[curr_tour_id].action
        };
        while is_direct_road(curr_tour_id, &self.curr_tour) {
            let action = self.curr_tour[curr_tour_id].action.clone();
            self.state.move_forward(&action);
            self.direct_road.push(action);
            self.curr_tour.pop();
            curr_tour_id += 1;
        }

        let mut leaf_id = 0;
        for edge in &self.curr_tour[curr_tour_id..] {
            match edge.property {
                EdgeProperty::Leaf => {
                    if self.buckets[leaf_id].is_empty() {
                        leaf_id += 1;
                        continue;
                    }
                    self.next_tour.push(Edge {
                        property: EdgeProperty::Forward,
                        action: edge.action.clone(),
                    });
                    for (action, evaluator, hash) in &self.buckets[leaf_id] {
                        self.next_tour.push(Edge {
                            property: EdgeProperty::Leaf,
                            action: action.clone(),
                        });
                        self.leaves.push((evaluator.clone(), *hash));
                    }
                    self.next_tour.push(Edge {
                        property: EdgeProperty::Backward,
                        action: edge.action.clone(),
                    });
                    self.buckets[leaf_id].clear();
                    leaf_id += 1;
                }
                EdgeProperty::Forward => {
                    self.next_tour.push(edge.clone());
                }
                EdgeProperty::Backward => {
                    if self.next_tour.last().unwrap().property == EdgeProperty::Forward {
                        self.next_tour.pop();
                    } else {
                        self.next_tour.push(edge.clone());
                    }
                }
            }
        }
        std::mem::swap(&mut self.curr_tour, &mut self.next_tour);
        self.next_tour.clear();
    }

    // get the path from the root
    fn get_path(&self, best_leaf_id: usize, turn: usize) -> Vec<Action> {
        // eprintln!("curr_tour.len() = {}", self.curr_tour.capacity());
        
        let mut ret = self.direct_road.clone();
        ret.reserve(turn);
        let mut leaf_id = 0;
        for edge in &self.curr_tour {
            match edge.property {
                EdgeProperty::Leaf => {
                    if leaf_id == best_leaf_id {
                        ret.push(edge.action.clone());
                        return ret;
                    }
                    leaf_id += 1;
                }
                EdgeProperty::Forward => ret.push(edge.action.clone()),
                EdgeProperty::Backward => { ret.pop(); },
            }
        }

        panic!("invalid argument: best_leaf_id");
    }
}

pub fn beam_search(config: &Config, state: State) -> Option<Vec<Action>> {
    let mut tree = Tree::new(state, config);
    let mut selector = Selector::new(config.beam_width);

    for turn in 0..config.max_turn {
        tree.dfs(&mut selector);

        if let Some(candidate) = &selector.finished_candidate {
            // find the feasible solution in turn minimizing problem
            let mut ret = tree.get_path(candidate.parent as usize, turn + 1);
            ret.push(candidate.action.clone());
            return Some(ret);
        }

        if selector.candidates.is_empty() {
            // cannot find the feasible solution
            return None;
        }

        if turn == config.max_turn - 1 {
            // the last turn
            let best_candidate = selector.get_best_candidate();
            let mut ret = tree.get_path(best_candidate.parent as usize, turn + 1);
            ret.push(best_candidate.action);
            return Some(ret);
        }

        tree.update(&selector.candidates);
        selector.clear();
    }

    unreachable!();
}
