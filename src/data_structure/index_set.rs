pub struct IndexSet {
    data: Vec<usize>,
    pos: Vec<usize>,
}

impl IndexSet {
    pub fn new(n: usize) -> Self {
        IndexSet {
            data: Vec::with_capacity(n),
            pos: vec![usize::MAX; n],
        }
    }

    pub fn push(&mut self, x: usize) -> bool {
        if self.pos[x] == usize::MAX {
            self.pos[x] = self.data.len();
            self.data.push(x);
            true
        } else {
            false
        }
    }

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

    pub fn contains(&self, x: usize) -> bool {
        self.pos[x] != usize::MAX
    }

    pub fn clear(&mut self) {
        for x in self.data.iter() {
            self.pos[*x] = usize::MAX;
        }
        self.data.clear();
    }
}
