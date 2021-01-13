use proconio::input;
use std::cmp::min;
use std::usize::MAX;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
    }

    let mut d = vec![vec![MAX; n + 1]; n + 1];

    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
            c: usize,
        }
        if c <= l {
            d[u][v] = min(d[u][v], c);
            d[v][u] = min(d[v][u], c);
        }
    }

    for k in 1..=n {
        for i in 1..=n {
            if i != k {
                for j in 1..=n {
                    if j != k && j != i {
                        if d[i][k] < MAX && d[k][j] < MAX && d[i][k] + d[k][j] <= l {
                            d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
                        }
                    }
                }
            }
        }
    }

    let mut ans = vec![vec![MAX; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            if d[i][j] <= l {
                ans[i][j] = 1;
            }
        }
    }

    for k in 1..=n {
        for i in 1..=n {
            if i != k {
                for j in 1..=n {
                    if j != k && j != i {
                        if ans[i][k] < MAX && ans[k][j] < MAX {
                            ans[i][j] = min(ans[i][j], ans[i][k] + ans[k][j]);
                        }
                    }
                }
            }
        }
    }

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }

        println!(
            "{}",
            if ans[u][v] == MAX {
                "-1".to_string()
            } else {
                (ans[u][v] - 1).to_string()
            }
        );
    }
}
