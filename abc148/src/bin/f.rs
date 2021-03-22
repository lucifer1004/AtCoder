use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Usize1;

fn shortest_distance(start: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = adj.len();
    let mut dist = vec![n + 1; n];
    dist[start] = 0;
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    while !q.is_empty() {
        let (u, d) = q.pop_front().unwrap();
        for v in adj[u].clone() {
            if dist[v] == n + 1 {
                dist[v] = d + 1;
                q.push_back((v, d + 1));
            }
        }
    }

    dist
}

fn main() {
    input!{
        n: usize,
        u: Usize1,
        v: Usize1,
        edges: [(Usize1, Usize1); n - 1],
    };

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let sdu = shortest_distance(u, &adj);
    let sdv = shortest_distance(v, &adj);

    let mut max_distance = 0;
    for i in 0..n {
        if sdv[i] > sdu[i] {
            max_distance = max_distance.max(sdv[i]);
        }
    }

    println!("{}", max_distance - 1);
}
