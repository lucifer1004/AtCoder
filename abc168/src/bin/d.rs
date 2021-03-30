use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input!{
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    };

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut ans = vec![n; n];
    let mut dq = VecDeque::new();
    dq.push_back(0);

    while !dq.is_empty() {
        let u = dq.pop_front().unwrap();
        for v in adj[u].clone() {
            if ans[v] == n {
                ans[v] = u;
                dq.push_back(v);
            }
        }
    }

    println!("Yes");
    for i in 1..n {
        println!("{}", ans[i] + 1);
    }
}
