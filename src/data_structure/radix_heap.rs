type Key = u64; // u32 or u64
const KEY_BITS: usize = 8 * std::mem::size_of::<Key>();

// incremental min heap
pub struct RadixHeap<Val> {
    vs: [Vec<(Key, Val)>; KEY_BITS + 1],
    ms: [Key; KEY_BITS + 1],
    s: usize,
    last: Key,
    buf: Vec<(Key, Val)>,
}

impl<Val> RadixHeap<Val>
where Val: Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        RadixHeap {
            vs: vec![Vec::new(); KEY_BITS + 1].try_into().unwrap(),
            ms: [Key::MAX; KEY_BITS + 1],
            s: 0,
            last: 0,
            buf: Vec::new(),
        }
    }

    fn bit_width(x: Key) -> usize {
        KEY_BITS - x.leading_zeros() as usize
    }

    pub fn push(&mut self, key: Key, val: Val) {
        self.s += 1;
        let b = Self::bit_width(key ^ self.last);
        self.vs[b].push((key, val));
        self.ms[b] = self.ms[b].min(key);
    }

    pub fn pop(&mut self) -> Option<(Key, Val)> {
        if self.s == 0 {
            return None;
        }
        if self.ms[0] == Key::MAX {
            let mut idx = 1;
            while self.ms[idx] == Key::MAX {
                idx += 1;
            }
            self.last = self.ms[idx];
            std::mem::swap(&mut self.buf, &mut self.vs[idx]);
            for (key, val) in &self.buf {
                let b = Self::bit_width(key ^ self.last);
                self.vs[b].push((key.clone(), val.clone()));
                self.ms[b] = self.ms[b].min(key.clone());
            }
            self.buf.clear();
            self.ms[idx] = Key::MAX;
        }
        self.s -= 1;
        let res = self.vs[0].pop().unwrap();
        if self.vs[0].is_empty() {
            self.ms[0] = Key::MAX;
        }
        Some(res)
    }
}
