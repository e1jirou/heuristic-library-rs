use crate::utility::ChangeMinMax;

// calculate the TSP optimal solution
// start from the last vertex (and end to the last vertex)
type Cost = i64;
pub fn solve_tsp(cost_matrix: &Vec<Vec<Cost>>, back_to_the_start_point: bool) -> Vec<usize> {
    if cost_matrix.len() == 1 {
        return vec![0];
    }
    let n = cost_matrix.len() - 1;
    let inf = Cost::max_value() / 2;

    // dp
    let mut dp = vec![vec![inf; 1 << n]; n];
    for v in 0..n {
        dp[v][1 << v] = cost_matrix[n][v];
    }
    for s in 1..(1 << n) {
        for u in 0..n {
            if ((s >> u) & 1) == 0 {
                continue;
            }
            for v in 0..n {
                if ((s >> v) & 1) == 0 {
                    let cost = dp[u][s] + cost_matrix[u][v];
                    dp[v][s | (1 << v)].chmin(cost);
                }
            }
        }
    }
    let mut last = n;
    let mut cost = inf;
    let mut s = (1 << n) - 1;
    for v in 0..n {
        if back_to_the_start_point {
            if cost.chmin(dp[v][s] + cost_matrix[v][n]) {
                last = v;
            }
        } else {
            if cost.chmin(dp[v][s]) {
                last = v;
            }
        }
    }
    let mut ret = vec![n; n + 1 + back_to_the_start_point as usize];
    ret[n] = last;
    s ^= 1 << last;
    for i in (1..n).rev() {
        for v in 0..n {
            if (s & (1 << v)) > 0 && dp[v][s] + cost_matrix[v][last] == dp[last][s | (1 << last)] {
                ret[i] = v;
                s ^= 1 << v;
                break;
            }
        }
        debug_assert_ne!(ret[i], n);
    }
    ret
}
