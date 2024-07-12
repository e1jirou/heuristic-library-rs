use std::ops::{Index, IndexMut};

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

impl<T> Index<usize> for ObjectPool<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for ObjectPool<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
