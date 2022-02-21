use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;

const KMAX: usize = 20;

fn dfs(u: usize, p: usize, adj: &Vec<Vec<usize>>, x: &Vec<usize>, heaviest: &mut Vec<Vec<usize>>) {
    let mut children = vec![];

    for &v in adj[u].iter() {
        if v != p {
            dfs(v, u, adj, x, heaviest);
            children.push(v);
        }
    }

    let mut pq: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    let mut ptr = vec![0; children.len()];
    for i in 0..children.len() {
        pq.push((heaviest[children[i]][0], i));
    }
    pq.push((x[u], children.len()));

    while !pq.is_empty() && heaviest[u].len() < KMAX {
        let (w, i) = pq.pop().unwrap();
        heaviest[u].push(w);
        if i < children.len() && ptr[i] + 1 < heaviest[children[i]].len() {
            ptr[i] += 1;
            pq.push((heaviest[children[i]][ptr[i]], i));
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; n],
        edges: [(Usize1, Usize1); n - 1],
        queries: [(Usize1, usize); q],
    }

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut heaviest = vec![vec![]; n];
    dfs(0, n, &adj, &x, &mut heaviest);

    for (u, k) in queries {
        println!("{}", heaviest[u][k - 1]);
    }
}
