use proconio::input;
use proconio::marker::Usize1;

const INF: usize = 2_000_000_000_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, usize); m],
    }

    let mut updated = vec![vec![false; n]; n];
    let mut dist = vec![vec![INF; n]; n];
    for &(u, v, w) in edges.iter() {
        dist[u][v] = dist[u][v].min(w);
        dist[v][u] = dist[v][u].min(w);
    }

    for k in 0..n {
        for i in 0..n {
            if i == k {
                continue;
            }

            for j in 0..n {
                if j == i || j == k {
                    continue;
                }

                if dist[i][k] + dist[k][j] <= dist[i][j] {
                    updated[i][j] = true;
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    let mut ans = 0;
    for &(u, v, w) in edges.iter() {
        if dist[u][v] < w || (dist[u][v] == w && updated[u][v]) {
            ans += 1;
        }
    }

    println!("{}", ans);
}
