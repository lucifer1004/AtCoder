use proconio::input;
use proconio::marker::Usize1;

fn dfs(u: usize, order: &mut Vec<usize>, vis: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
    vis[u] = true;
    order.push(u);
    for i in 0..adj[u].len() {
        if !vis[adj[u][i]] {
            dfs(adj[u][i], order, vis, adj);
        }
    }
}

fn solve(i: usize, ans: &mut usize, start: &Vec<bool>, order: &Vec<usize>, color: &mut Vec<usize>, adj: &Vec<Vec<usize>>) {
    if i == order.len() || start[order[i]] {
        *ans += 1;
    } else {
        let u = order[i];
        let mut can = vec![true; 4];
        for j in 0..adj[u].len() {
            can[color[adj[u][j]]] = false;
        }
        for c in 1..=3 {
            if can[c] {
                color[u] = c;
                solve(i + 1, ans, start, order, color, adj);
                color[u] = 0;
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut vis = vec![false; n];
    let mut order = vec![];
    let mut start = vec![false; n];

    for i in 0..n {
        if !vis[i] {
            start[i] = true;
            dfs(i, &mut order, &mut vis, &adj);
        }
    }

    let mut ans = 1;
    let mut color = vec![0; n];
    for i in 0..n {
        if start[order[i]] {
            let mut sub = 0;
            start[order[i]] = false;
            solve(i, &mut sub, &start, &order, &mut color, &adj);
            ans *= sub;
        }
    }

    println!("{}", ans);
}
