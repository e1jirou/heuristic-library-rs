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
        let capacity = rng.gen_range((1 * capacity)..(2 * capacity));
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
            None => None,
        }
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut V> {
        let group = self.hash(key);
        match Self::find(key, &self.data[group]) {
            Some(i) => Some(&mut self.data[group][i].1),
            None => None,
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

pub struct OpenAddressHashMap<V> {
    capacity: usize,
    xor: usize,
    len: usize,
    data: Vec<(usize, V)>,
}

impl<V: Clone + Default> OpenAddressHashMap<V> {
    pub fn new(capacity: usize, seed: u64) -> Self {
        use rand::{Rng, SeedableRng};
        let mut rng = rand_pcg::Mcg128Xsl64::seed_from_u64(seed);
        let capacity = rng.gen_range((2 * capacity)..(4 * capacity));
        let xor = rng.gen_range(0..=usize::MAX);
        Self {
            capacity,
            xor,
            len: 0,
            data: vec![(usize::MAX, V::default()); capacity],
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        self.data.fill((usize::MAX, V::default()));
        self.len = 0;
    }

    fn hash(&self, key: usize) -> usize {
        (key ^ self.xor) % self.capacity
    }

    fn find(&self, key: usize, mut i: usize) -> Result<usize, usize> {
        while self.data[i].0 != key {
            if self.data[i].0 == usize::MAX {
                return Err(i);
            }
            i += 1;
            if i == self.capacity {
                i = 0;
            }
        }
        Ok(i)
    }

    pub fn contains_key(&self, key: usize) -> bool {
        let i = self.hash(key);
        self.find(key, i).is_ok()
    }

    pub fn get(&self, key: usize) -> Option<&V> {
        let i = self.hash(key);
        match self.find(key, i) {
            Ok(i) => Some(&self.data[i].1),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut V> {
        let i = self.hash(key);
        match self.find(key, i) {
            Ok(i) => Some(&mut self.data[i].1),
            Err(_) => None,
        }
    }

    pub fn insert(&mut self, key: usize, mut value: V) -> Option<V> {
        let i = self.hash(key);
        match self.find(key, i) {
            Ok(i) => {
                std::mem::swap(&mut self.data[i].1, &mut value);
                Some(value)
            }
            Err(i) => {
                self.data[i] = (key, value);
                self.len += 1;
                debug_assert_ne!(self.len, self.capacity);
                None
            }
        }
    }
}
