pub struct ChainHashMap<V> {
    capacity: usize,
    xor: usize,
    len: usize,
    data: Vec<Vec<(usize, V)>>,
}

impl<V: Clone> ChainHashMap<V> {
    pub fn new(capacity: usize, seed: u64) -> Self {
        use rand::{Rng, SeedableRng};
        let mut rng = rand_pcg::Mcg128Xsl64::seed_from_u64(seed);
        let capacity = rng.gen_range((4 * capacity)..(8 * capacity));
        let xor = rng.gen_range(0..=usize::MAX);
        Self {
            capacity,
            xor,
            len: 0,
            data: vec![vec![]; capacity],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        for list in &mut self.data {
            list.clear();
        }
        self.len = 0;
    }

    fn hash(&self, key: usize) -> usize {
        (key ^ self.xor) % self.capacity
    }

    fn find(key: usize, list: &[(usize, V)]) -> Option<usize> {
        list.iter().position(|x| x.0 == key)
    }

    pub fn contains_key(&self, key: usize) -> bool {
        let group = self.hash(key);
        Self::find(key, &self.data[group]).is_some()
    }

    pub fn get(&self, key: usize) -> Option<&V> {
        let group = self.hash(key);
        match Self::find(key, &self.data[group]) {
            Some(i) => Some(&self.data[group][i].1),
            None => None
        }
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut V> {
        let group = self.hash(key);
        match Self::find(key, &self.data[group]) {
            Some(i) => Some(&mut self.data[group][i].1),
            None => None
        }
    }

    pub fn insert(&mut self, key: usize, mut value: V) -> Option<V> {
        let group = self.hash(key);
        match Self::find(key, &self.data[group]) {
            Some(i) => {
                std::mem::swap(&mut self.data[group][i].1, &mut value);
                Some(value)
            }
            None => {
                self.data[group].push((key, value));
                self.len += 1;
                None
            }
        }
    }

    pub fn remove(&mut self, key: usize) -> Option<V> {
        let group = self.hash(key);
        match Self::find(key, &self.data[group]) {
            Some(i) => {
                self.len -= 1;
                Some(self.data[group].swap_remove(i).1)
            }
            None => None,
        }
    }
}
