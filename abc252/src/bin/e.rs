use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, i64); m],
    }

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for (i, &(u, v, _)) in edges.iter().enumerate() {
        adj[u].push(i);
        adj[v].push(i);
    }
    let mut used = vec![false; m];
    let mut dist = vec![1_000_000_000_000_000_000i64; n];
    dist[0] = 0;
    let mut pq: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
    pq.push((0, 0, 0));
    while !pq.is_empty() {
        let (cost, u, e) = pq.pop().unwrap();
        let cost = -cost;
        if cost > dist[u] {
            continue;
        }

        if u != 0 {
            used[e] = true;
        }
        for &i in adj[u].iter() {
            let (u1, v1, w) = edges[i];
            let v = u1 + v1 - u;
            if cost + w < dist[v] {
                dist[v] = cost + w;
                pq.push((-dist[v], v, i));
            }
        }
    }

    for i in 0..m {
        if used[i] {
            print!("{} ", i + 1);
        }
    }
}
