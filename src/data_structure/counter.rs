struct Counter<T> {
    map: std::collections::HashMap<T, usize>,
}

impl<T: Eq + std::hash::Hash> Counter<T> {
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: std::collections::HashMap::with_capacity(capacity),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.map.reserve(additional);
    }

    pub fn insert(&mut self, key: T) {
        *self.map.entry(key).or_insert(0) += 1;
    }

    pub fn remove(&mut self, key: &T) -> bool {
        if let Some(count) = self.map.get_mut(key) {
            *count -= 1;
            if *count == 0 {
                self.map.remove(key);
            }
            true
        } else {
            false
        }
    }

    pub fn count(&self, key: &T) -> usize {
        *self.map.get(key).unwrap_or(&0)
    }
}
