use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
    }

    let mut dp = vec![vec![0; 2]; 3 * n + 3];
    dp[0][0] = 1;
    dp[1][1] = 1;

    for i in 1..n {
        let mut ndp = vec![vec![0; 2]; 3 * n + 3];
        for removed in 0..=n {
            // need_vert = 0 (do not need)
            if dp[removed][0] > 0 {
                // do not remove any
                ndp[removed][0] += dp[removed][0];
                ndp[removed][0] %= p;

                // remove current vert or any horiz
                ndp[removed + 1][0] += dp[removed][0] * 3;
                ndp[removed + 1][0] %= p;

                // remove vert and any horiz
                ndp[removed + 2][1] += dp[removed][0] * 2;
                ndp[removed + 2][1] %= p;
            }

            // need_vert = 1 (need)
            if dp[removed][1] > 0 {
                // do not remove any
                ndp[removed][0] += dp[removed][1];
                ndp[removed][0] %= p;

                // remove vert unless this is the last position
                if i != n - 1 {
                    ndp[removed + 1][1] += dp[removed][1];
                    ndp[removed + 1][1] %= p;
                }
            }
        }

        dp = ndp;
    }

    for i in 1..n {
        print!("{} ", dp[i][0]);
    }
}
