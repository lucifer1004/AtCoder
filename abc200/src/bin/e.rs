use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
    }

    let mut dp = vec![vec![0usize; 3 * n + 1]; 3];
    for i in 1..=n {
        dp[0][i] = i;
    }
    for j in 2..=3 {
        for i in j..=n * j {
            dp[j - 1][i] = dp[j - 2][(i - 1).min(n * (j - 1))]
                - dp[j - 2][if i > n { i - n - 1 } else { 0 }]
                + dp[j - 1][i - 1];
        }
    }

    let mut lo = 3;
    let mut hi = 3 * n;
    while lo <= hi {
        let mid = (lo + hi) >> 1;
        if dp[2][mid] < k {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    let sum = lo;
    k -= dp[2][sum - 1];

    for a in 1..=n {
        let b_plus_c = sum - a;
        if b_plus_c > 2 * n {
            continue;
        }
        if k > dp[1][b_plus_c] - dp[1][b_plus_c - 1] {
            k -= dp[1][b_plus_c] - dp[1][b_plus_c - 1];
        } else {
            let b = if b_plus_c <= n { 1 } else { b_plus_c - n } + k - 1;
            let c = b_plus_c - b;
            println!("{} {} {}", a, b, c);
            break;
        }
    }
}
