use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];
    for (a, b) in edges {
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut ans = vec![1_000_000_000usize; 1 << n];
    let mut vis = vec![vec![false; n]; 1 << n];
    let mut q = VecDeque::new();
    ans[0] = 0;
    for i in 0..n {
        ans[1 << i] = 1;
        q.push_back((1 << i, i, 1));
        vis[1 << i][i] = true;
    }

    while !q.is_empty() {
        let (mask, node, dist) = q.pop_front().unwrap();

        for &next in &adj[node] {
            let next_mask = mask ^ (1 << next);
            if ans[next_mask] > dist + 1 {
                ans[next_mask] = dist + 1;
            }
            if !vis[next_mask][next] {
                q.push_back((next_mask, next, dist + 1));
                vis[next_mask][next] = true;
            }
        }
    }

    let mut sum = 0;
    for i in 0..(1 << n) {
        sum += ans[i];
    }

    println!("{}", sum);
}
