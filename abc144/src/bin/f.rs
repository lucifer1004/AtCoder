use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
        }
        adj[u].push(v);
    }

    let mut ans = 1e18;

    // My original idea is to save two values for each node, "blocked" and "non-blocked".
    // However, this is wrong, because both nodes can be blocked at the same edge.
    // Instead, we need to enumerate the nodes, and try blocking its heaviest edge.
    for block in 1..n {
        let mut dp = vec![0.; n + 1];
        for u in (1..n).rev() {
            let mut num = adj[u].len();
            let mut hi = 0.;
            let mut tot = 0.;
            for v in adj[u].clone() {
                if dp[v] > hi {
                    hi = dp[v];
                }
                tot += dp[v];
            }
            if u == block && num >= 2 {
                num -= 1;
                tot -= hi;
            }
            dp[u] = tot / (num as f64) + 1.;
        }
        if dp[1] < ans {
            ans = dp[1];
        }
    }

    println!("{}", ans);
}
