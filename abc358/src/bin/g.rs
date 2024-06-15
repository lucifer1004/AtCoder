use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        si: Usize1,
        sj: Usize1,
        a: [[isize; w]; h],
    }

    let mut dp = vec![vec![-1isize; w]; h];
    let mut ans = 0;
    dp[si][sj] = 0;
    for t in 0..=k.min(h * w) {
        let mut new_dp = vec![vec![-1isize; w]; h];
        for i in 0..h {
            for j in 0..w {
                if dp[i][j] == -1 {
                    continue;
                }

                ans = ans.max(dp[i][j] + a[i][j] * (k as isize - t as isize));

                if i > 0 {
                    new_dp[i - 1][j] = new_dp[i - 1][j].max(dp[i][j] + a[i - 1][j]);
                }
                if i + 1 < h {
                    new_dp[i + 1][j] = new_dp[i + 1][j].max(dp[i][j] + a[i + 1][j]);
                }
                if j > 0 {
                    new_dp[i][j - 1] = new_dp[i][j - 1].max(dp[i][j] + a[i][j - 1]);
                }
                if j + 1 < w {
                    new_dp[i][j + 1] = new_dp[i][j + 1].max(dp[i][j] + a[i][j + 1]);
                }
            }
        }
        dp = new_dp;
    }

    println!("{}", ans);
}
