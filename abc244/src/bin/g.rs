use proconio::input;
use proconio::marker::Usize1;

fn dfs(u: usize, p: usize, target: &[usize], adj: &[Vec<usize>], vis: &mut [bool], now: &mut [usize], path: &mut Vec<usize>) {
    now[u] ^= 1;
    path.push(u);
    vis[u] = true;
    for &v in adj[u].iter() {
        if vis[v] {
            continue;
        }

        vis[v] = true;
        if now[v] == 0 {
            dfs(v, u, target, adj, vis, now, path);
            path.push(u);
            now[u] ^= 1;
        }
    }

    if p < target.len() && now[u] != target[u] {
        path.push(p);
        path.push(u);
        now[u] ^= 1;
        now[p] ^= 1;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
        s: String,
    }

    let target = s.chars()
        .into_iter()
        .map(|c| c as u8 as usize - '0' as u8 as usize)
        .collect::<Vec<_>>();

    let mut adj = vec![vec![]; n];
    for &(u, v) in &edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut now = vec![0; n];
    let mut vis = vec![false; n];
    let mut path = vec![];
    dfs(0, n, &target, &adj, &mut vis, &mut now, &mut path);

    let start = if now[0] == target[0] { 0 } else { 1 };

    println!("{}", path.len() - start);
    for i in start..path.len() {
        print!("{} ", path[i] + 1);
    }
}
