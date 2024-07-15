fn sa_naive<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&l, &r| {
        let mut l = l;
        let mut r = r;
        if l == r {
            return std::cmp::Ordering::Equal;
        }
        while l < n && r < n {
            if s[l] != s[r] {
                return s[l].cmp(&s[r])
            }
            l += 1;
            r += 1;
        }
        if l == n {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    sa
}

fn sa_doubling(s: &[usize]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rnk = s.to_vec();
    let mut tmp = vec![0; n];
    let mut k = 1;
    while k < n {
        let cmp = |&x: &usize, &y: &usize| {
            if rnk[x] != rnk[y] {
                return rnk[x].cmp(&rnk[y]);
            }
            let rx = if x + k < n {
                rnk[x + k] as i64
            } else {
                -1
            };
            let ry = if y + k < n {
                rnk[y + k] as i64
            } else {
                -1
            };
            rx.cmp(&ry)
        };
        sa.sort_by(cmp);
        tmp[sa[0]] = 0;
        for i in 1..n {
            tmp[sa[i]] = tmp[sa[i - 1]] + if cmp(&sa[i - 1], &sa[i]).is_lt() { 1 } else { 0 };
        }
        std::mem::swap(&mut tmp, &mut rnk);
        k *= 2;
    }
    sa
}

// SA-IS, linear-time suffix array construction
// Reference:
// G. Nong, S. Zhang, and W. H. Chan,
// Two Efficient Algorithms for Linear Time Suffix Array Construction
fn sa_is(s: &[usize], upper: usize) -> Vec<usize> {
    const THRESHOLD_NAIVE: usize = 10;
    const THRESHOLD_DOUBLING: usize = 40;

    let n = s.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![0];
    }
    if n == 2 {
        if s[0] < s[1] {
            return vec![0, 1];
        } else {
            return vec![1, 0];
        }
    }
    if n < THRESHOLD_NAIVE {
        return sa_naive(s);
    }
    if n < THRESHOLD_DOUBLING {
        return sa_doubling(s);
    }

    // offset 1
    let mut sa = vec![0; n];
    let mut ls = vec![false; n];
    for i in (0..(n - 1)).rev() {
        ls[i] = if s[i] == s[i + 1] {
            ls[i + 1]
        } else {
            s[i] < s[i + 1]
        }
    }
    let mut sum_l = vec![0; upper + 1];
    let mut sum_s = vec![0; upper + 1];
    for i in 0..n {
        if !ls[i] {
            sum_s[s[i]] += 1;
        } else {
            sum_l[s[i] + 1] += 1;
        }
    }
    for i in 0..=upper {
        sum_s[i] += sum_l[i];
        if i < upper {
            sum_l[i + 1] += sum_s[i];
        }
    }

    let induce = |sa: &mut [usize], lms: &[usize]| {
        sa.fill(0);
        let mut buf = sum_s.to_vec();
        for &d in lms {
            if d == n {
                continue;
            }
            sa[buf[s[d]]] = d + 1;
            buf[s[d]] += 1;
        }
        buf.copy_from_slice(&sum_l);
        sa[buf[s[n - 1]]] = n;
        buf[s[n - 1]] += 1;
        for i in 0..n {
            let v = sa[i];
            if v >= 2 && !ls[v - 2] {
                sa[buf[s[v - 2]]] = v - 1;
                buf[s[v - 2]] += 1;
            }
        }
        buf.copy_from_slice(&sum_l);
        for i in (0..n).rev() {
            let v = sa[i];
            if v >= 2 && ls[v - 2] {
                buf[s[v - 2] + 1] -= 1;
                sa[buf[s[v - 2] + 1]] = v - 1;
            }
        }
    };

    // offset 1
    let mut lms_map = vec![0; n + 1];
    let mut m = 0;
    for i in 1..n {
        if !ls[i - 1] && ls[i] {
            m += 1;
            lms_map[i] = m;
        }
    }
    let mut lms = Vec::with_capacity(m);
    for i in 1..n {
        if !ls[i - 1] && ls[i] {
            lms.push(i);
        }
    }

    induce(&mut sa, &lms);

    if m > 0 {
        let mut sorted_lms = Vec::with_capacity(m);
        for &v in &sa {
            if lms_map[v - 1] != 0 {
                sorted_lms.push(v - 1);
            }
        }
        let mut rec_s = vec![0; m];
        let mut rec_upper = 0;
        rec_s[lms_map[sorted_lms[0]] - 1] = 0;
        for i in 1..m {
            let mut l = sorted_lms[i - 1];
            let mut r = sorted_lms[i];
            let end_l = if lms_map[l] < m {
                lms[lms_map[l]]
            } else {
                n
            };
            let end_r = if lms_map[r] < m {
                lms[lms_map[r]]
            } else {
                n
            };
            let mut same = true;
            if end_l - l != end_r - r {
                same = false;
            } else {
                while l < end_l {
                    if s[l] != s[r] {
                        break;
                    }
                    l += 1;
                    r += 1;
                }
                if l == n || s[l] != s[r] {
                    same = false;
                }
            }
            if !same {
                rec_upper += 1;
            }
            rec_s[lms_map[sorted_lms[i]] - 1] = rec_upper;
        }

        let rec_sa = sa_is(&rec_s, rec_upper);
        for i in 0..m {
            sorted_lms[i] = lms[rec_sa[i]];
        }
        induce(&mut sa, &sorted_lms);
    }
    for i in sa.iter_mut() {
        *i -= 1;
    }
    sa
}

pub fn suffix_array<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&l, &r| s[l].cmp(&s[r]));
    let mut s2 = vec![0; n];
    let mut now = 0;
    for i in 0..n {
        if i > 0 && s[idx[i - 1]] != s[idx[i]] {
            now += 1;
        }
        s2[idx[i]] = now;
    }
    sa_is(&s2, now)
}
