struct Node {
    children: [usize; 26],
}

impl Node {
    fn new() -> Self {
        Self {
            children: [usize::MAX; 26],
        }
    }
}

pub struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new()]
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.nodes.reserve(additional);
    }

    pub fn insert(&mut self, s: &[usize]) {
        debug_assert!(s.iter().all(|&c| c < 26));
        let mut v = 0;
        for i in 0..s.len() {
            if self.nodes[v].children[s[i]] == usize::MAX {
                for j in i..s.len() {
                    self.nodes[v].children[s[j]] = self.nodes.len();
                    v = self.nodes.len();
                    self.nodes.push(Node::new());
                }
                break;
            } else {
                v = self.nodes[v].children[s[i]];
            }
        }
    }
}
