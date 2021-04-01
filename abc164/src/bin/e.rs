use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::Usize1;

const INF: i64 = 1_000_000_000_000_000_000;
const LIMIT: usize = 2500;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        edges: [(Usize1, Usize1, usize, i64); m],
        ex: [(usize, i64); n],
    }

    let s = s.min(LIMIT);
    let mut adj = vec![vec![]; n];
    for (u, v, cost, time) in edges {
        adj[u].push((v, cost, time));
        adj[v].push((u, cost, time));
    }
    let mut dist = vec![vec![INF; LIMIT + 1]; n];
    dist[0][s] = 0;
    let mut pq: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
    pq.push((0, 0, s));

    while !pq.is_empty() {
        let (t, u, s) = pq.pop().unwrap();
        let t = -t;
        if t > dist[u][s] {
            continue;
        }
        let mut ex_num = 1;
        while s + (ex_num - 1) * ex[u].0 < LIMIT {
            let ns = (s + ex_num * ex[u].0).min(LIMIT);
            let nt = t + ex[u].1 * ex_num as i64;
            if nt < dist[u][ns] {
                dist[u][ns] = nt;
                pq.push((-nt, u, ns));
            }
            ex_num += 1;
        }
        for (v, cost, time) in adj[u].clone() {
            if s >= cost && t + time < dist[v][s - cost] {
                dist[v][s - cost] = t + time;
                pq.push((-t - time, v, s - cost));
            }
        }
    }

    for i in 1..n {
        let mut ans = INF;
        for j in 0..=LIMIT {
            ans = ans.min(dist[i][j]);
        }
        println!("{}", ans);
    }
}
