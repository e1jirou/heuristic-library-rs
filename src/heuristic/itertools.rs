pub fn next_permutation<T: Ord>(p: &mut [T]) -> bool {
    let n = p.len();
    if n <= 1 {
        return false;
    }
    for i in (0..(n - 1)).rev() {
        if p[i] < p[i + 1] {
            for j in (i..n).rev() {
                if p[i] >= p[j] {
                    continue;
                }
                p.swap(i, j);
                p[(i + 1)..].reverse();
                return true;
            }
        }
    }
    false
}
