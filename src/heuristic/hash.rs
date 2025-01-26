pub fn xorshift64(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

pub fn dhondt_hash(votes: &[u64], num_seats: usize) -> u64 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    let mut seats = vec![0; votes.len()];
    let mut que = BinaryHeap::from_iter(votes.iter().enumerate().map(|(i, &x)| Reverse((x, i))));
    for _ in 0..num_seats {
        if let Some(mut top) = que.peek_mut() {
            let party = top.0.1;
            seats[party] += 1;
            *top = Reverse((votes[party] / (seats[party] + 1), party));
        }
    }
    let mut hash = 0;
    for &x in &seats {
        hash = xorshift64(hash) ^ x;
    }
    hash
}
