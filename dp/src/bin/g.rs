use proconio::input;
use proconio::marker::Usize1;
use std::cmp::max;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    let mut in_degree: Vec<usize> = vec![0; n];
    for (u, v) in edges {
        adj[u].push(v);
        in_degree[v] += 1;
    }

    let mut que: VecDeque<usize> = VecDeque::new();
    let mut topo: Vec<usize> = vec![];

    for u in 0..n {
        if in_degree[u] == 0 {
            que.push_back(u);
        }
    }

    while !que.is_empty() {
        let u = que.pop_front().unwrap();
        topo.push(u);
        for v in adj[u].clone() {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                que.push_back(v);
            }
        }
    }

    let mut rank: Vec<usize> = vec![0; n];
    for i in 0..n {
        rank[topo[i]] = i;
    }

    let mut ans = 0usize;
    let mut dp: Vec<usize> = vec![0; n];

    for i in (0..n).rev() {
        let u = topo[i];
        for v in adj[u].clone() {
            dp[i] = max(dp[i], dp[rank[v]] + 1);
        }
        ans = max(ans, dp[i]);
    }

    println!("{}", ans);
}
