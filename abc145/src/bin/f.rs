use proconio::input;

const INF: usize = 1_000_000_000_000_000;

fn calc(x: usize, y: usize) -> usize {
    if x > y {
        0
    } else {
        y - x
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }

    h.insert(0, 0);
    let mut distinct_heights = h.clone();
    distinct_heights.sort();
    distinct_heights.dedup();
    let m = distinct_heights.len();

    let mut mapping = std::collections::HashMap::new();
    for i in 0..m {
        mapping.insert(distinct_heights[i], i);
    }

    let mut dp = vec![vec![INF; k + 1]; m];

    dp[0][0] = 0;

    for i in 1..=n {
        let mut ndp = vec![vec![INF; k + 1]; m];
        let now = mapping[&h[i]];
        for last in 0..m {
            for j in 0..=k {
                if dp[last][j] < INF {
                    ndp[now][j] = ndp[now][j].min(dp[last][j] + calc(distinct_heights[last], h[i]));
                    if j < k && distinct_heights[last] != h[i] {
                        ndp[last][j + 1] = ndp[last][j + 1].min(dp[last][j]);
                    }
                }
            }
        }
        std::mem::swap(&mut dp, &mut ndp);
    }

    let mut ans = INF;
    for last in 0..m {
        for j in 0..=k {
            ans = ans.min(dp[last][j]);
        }
    }

    println!("{}", ans);
}
