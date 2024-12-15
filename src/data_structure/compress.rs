pub struct Compress<T> {
    data: Vec<T>,
}

impl<T: Clone + Copy + Ord> Compress<T> {
    pub fn new() -> Self {
        Compress {
            data: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Compress {
            data: Vec::with_capacity(capacity)
        }
    }

    pub fn push(&mut self, x: T) {
        self.data.push(x);
    }

    pub fn build(&mut self) {
        self.data.sort();
        self.data.dedup();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, x: T) -> Result<usize, usize> {
        self.data.binary_search(&x)
    }

    pub fn bisect(&self, x: T) -> usize {
        self.data.partition_point(|&y| y < x)
    }
}
