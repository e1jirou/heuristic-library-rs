pub struct Compress<T> {
    data: Vec<T>,
}

impl<T: Clone + Copy + Ord> Compress<T> {
    pub fn new(mut data: Vec<T>) -> Self {
        data.sort();
        data.dedup();
        Compress {
            data,
        }
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
