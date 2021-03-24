use proconio::input;
use proconio::marker::Usize1;

const INF: usize = 1_000_000_000;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
        m: usize,
        restrictions: [(Usize1, Usize1); m],
    };

    let mut d = vec![vec![INF; n]; n];
    for (u, v) in edges.clone() {
        d[u][v] = 1;
        d[v][u] = 1;
    }
    for k in 0..n {
        d[k][k] = 0;
        for i in 0..n {
            if i != k {
                for j in i + 1..n {
                    if j != k {
                        if d[i][k] + d[k][j] < d[i][j] {
                            d[i][j] = d[i][k] + d[k][j];
                            d[j][i] = d[i][j];
                        }
                    }
                }
            }
        }
    }

    let state_num = 1 << m;
    let mut dp = vec![0usize; state_num];
    dp[0] = 1;

    for i in 0..n - 1 {
        let mut inc = 0;
        let (u, v) = edges[i];

        for j in 0..m {
            let (p, q) = restrictions[j];
            if d[p][q] == (d[p][u] + 1 + d[v][q]).min(d[p][v] + 1 + d[u][q]) {
                inc ^= 1 << j;
            }
        }

        for j in (0..state_num).rev() {
            dp[j | inc] += dp[j];
        }
    }

    println!("{}", dp[state_num - 1]);
}
