use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::Usize1;

const INF: i64 = 1_000_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, i64, i64); m],
    }

    let mut adj = vec![vec![]; n];
    for (u, v, c, d) in edges {
        adj[u].push((v, c, d));
        adj[v].push((u, c, d));
    }

    let mut dist = vec![INF; n];
    dist[0] = 0;
    let mut pq: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    pq.push((0, 0));
    while !pq.is_empty() {
        let (d, u) = pq.pop().unwrap();
        let d = -d;
        if d > dist[u] {
            continue;
        }
        for &(v, ci, di) in adj[u].iter() {
            let mut nd = d + ci + di / (d + 1);
            let sqrt = (di as f64).sqrt().floor() as i64;
            if sqrt - 1 >= d {
                nd = nd.min(sqrt + ci - 1 + di / sqrt);
            }
            if sqrt >= d {
                nd = nd.min(sqrt + ci + di / (sqrt + 1));
            }
            if nd < dist[v] {
                dist[v] = nd;
                pq.push((-nd, v));
            }
        }
    }

    println!("{}", if dist[n - 1] < INF { dist[n - 1] } else { -1 });
}
