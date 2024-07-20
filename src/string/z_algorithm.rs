pub fn z_algorithm<T: Eq>(s: &Vec<T>) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return Vec::new();
    }
    let mut z = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        z[i] = if j + z[j] <= i {
            0
        } else {
            (j + z[j] - i).min(z[i - j])
        };
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if j + z[j] < i + z[i] {
            j = i;
        }
    }
    z[0] = n;
    z
}
