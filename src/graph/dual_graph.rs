use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct DualGraph {
    pub f: usize,
    pub edges: Vec<(usize, usize, usize, usize)>,
}

impl DualGraph {
    pub fn new(n: usize, m: usize, edges: &Vec<Vec<usize>>, points: &[(i32, i32)]) -> Self {
        let mut g_edges = edges.clone();
        for u in 0..n {
            // sort by polar angle
            let (ux, uy) = points[u];
            g_edges[u].sort_by_key(|&v| {
                let (vx, vy) = points[v];
                (1e18 * ((vy - uy) as f64).atan2((vx - ux) as f64)) as i64
            });
        }
        // make polygons
        let mut visited = HashSet::with_capacity(m);
        let f = m - n + 2; // Euler's polyhedral formula
        let mut polygons = Vec::with_capacity(f);
        for u in 0..n {
            for &v in &g_edges[u] {
                if visited.contains(&(n * u + v)) {
                    continue;
                }
                let mut polygon = Vec::with_capacity(3);
                let mut u = u;
                let mut v = v;
                while visited.insert(n * u + v) {
                    polygon.push(u);
                    let i = g_edges[v].iter().find_position(|&&w| w == u).unwrap().0;
                    let j = if i + 1 < g_edges[v].len() { i + 1 } else { 0 };
                    (u, v) = (v, g_edges[v][j]);
                }
                polygons.push(polygon);
            }
        }
        polygons.sort_by(|p, q| q.len().cmp(&p.len()));
        // polygons[0] may be the outer plane

        assert_eq!(polygons.len(), f);

        // make edges in dual graph
        let mut edges = Vec::with_capacity(m);
        let mut edge_sharing = HashMap::with_capacity(m);
        for (p, polygon) in polygons.iter().enumerate() {
            for i in 0..polygon.len() {
                let u = polygon[i];
                let v = polygon[if i + 1 < polygon.len() { i + 1 } else { 0 }];
                let edge_id = if u < v {
                    n * u + v
                } else {
                    n * v + u
                };
                match edge_sharing.get(&edge_id) {
                    Some(&q) => edges.push((p, q, u, v)),
                    None => {
                        edge_sharing.insert(edge_id, p);
                    },
                }
            }
        }
        Self { f, edges }
    }
}
