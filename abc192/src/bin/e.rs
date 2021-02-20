use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::Usize1;

const INF: i64 = 1_000_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
        trains: [(Usize1, Usize1, i64, i64); m],
    }

    let mut adj = vec![vec![]; n];
    for (a, b, t, k) in trains {
        adj[a].push((b, t.clone(), k.clone()));
        adj[b].push((a, t, k));
    }

    let mut dist = vec![INF; n];
    dist[x] = 0;
    let mut pq: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    pq.push((0, x));

    while !pq.is_empty() {
        let (tc, u) = pq.pop().unwrap();
        let tc = -tc;
        if tc > dist[u] {
            continue;
        }
        for (v, t, k) in adj[u].clone() {
            let nt = if tc % k == 0 {
                tc + t
            } else {
                (tc / k + 1) * k + t
            };
            if nt < dist[v] {
                dist[v] = nt;
                pq.push((-nt, v));
            }
        }
    }

    println!("{}", if dist[y] == INF {-1} else {dist[y]});
}
