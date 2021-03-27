use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Usize1;

const INF: usize = 1_000_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, char); m],
    }

    let mut adj: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; 26]; n];
    let mut adj_matrix = vec![vec![false; n]; n];
    for (u, v, c) in edges {
        let idx = (c as u8 - 'a' as u8) as usize;
        adj[u][idx].push(v);
        if u != v {
            adj[v][idx].push(u);
        }
        adj_matrix[u][v] = true;
        adj_matrix[v][u] = true;
    }

    let mut queue = VecDeque::new();
    queue.push_back((0usize, n - 1, 0));
    let mut vis = vec![vec![false; n]; n];
    vis[0][n - 1] = true;

    let mut ans = INF;

    while !queue.is_empty() {
        let (u, v, len) = queue.pop_front().unwrap();
        if len >= ans {
            break;
        }
        if adj_matrix[u][v] {
            ans = ans.min(len + 1);
            break;
        }
        for i in 0..26 {
            for j in 0..adj[u][i].len() {
                let nu = adj[u][i][j];
                for k in 0..adj[v][i].len() {
                    let nv = adj[v][i][k];
                    if nu == nv {
                        ans = ans.min(len + 2);
                    } else if !vis[nu][nv] {
                        vis[nu][nv] = true;
                        queue.push_back((nu, nv, len + 2));
                    }
                }
            }
        }
    }

    println!("{}", if ans == INF { "-1".to_string() } else { ans.to_string() });
}
