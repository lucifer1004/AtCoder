use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
    }
    let mut ans = 0;
    for i in 0..n {
        let mut q = VecDeque::new();
        let mut vis = vec![false; n];
        vis[i] = true;
        q.push_back(i);
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            ans += 1;
            for &v in adj[u].iter() {
                if !vis[v] {
                    vis[v] = true;
                    q.push_back(v);
                }
            }
        }
    }

    println!("{}", ans);
}
