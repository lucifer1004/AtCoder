use proconio::input;
use proconio::marker::Usize1;

use std::collections::BinaryHeap;

const INF: i32 = 1_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [i32; n],
        edges: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut pq: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    let mut cost = vec![INF; n];
    cost[0] = 0;
    pq.push((0, 0));

    while !pq.is_empty() {
        let (c, u) = pq.pop().unwrap();
        let c = -c;
        if c > cost[u] {
            continue;
        }

        for &v in adj[u].iter() {
            let e = if h[v] > h[u] { h[v] - h[u] } else { 0 };
            if c + e < cost[v] {
                cost[v] = c + e;
                pq.push((-cost[v], v));
            }
        }
    }

    let mut ans = 0;
    for i in 1..n {
        if h[i] < h[0] {
            ans = ans.max(h[0] - h[i] - cost[i]);
        }
    }

    println!("{}", ans);
}
