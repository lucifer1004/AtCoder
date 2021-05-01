use proconio::input;

const INF: usize = 1_000_000_000;

fn main() {
    input! {
        n: usize,
        a: [[usize; 5]; n],
    }

    let mut lo = 1;
    let mut hi = INF;
    while lo <= hi {
        let mid = (lo + hi) >> 1;
        let mut dp = vec![INF; 32];
        dp[0] = 0;
        for i in 0..n {
            let mut now = 0;
            for j in 0..5 {
                if a[i][j] >= mid {
                    now ^= 1 << j;
                }
            }
            for j in (0..32).rev() {
                dp[j | now] = dp[j | now].min(dp[j] + 1);
            }
        }

        if dp[31] <= 3 {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    println!("{}", hi);
}