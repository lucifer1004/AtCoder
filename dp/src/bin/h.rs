use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }
    let modulo = 1000000007usize;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; w]; h];
    if grid[0][0] == '.' {
        dp[0][0] = 1;
    }
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '.' {
                if i > 0 {
                    dp[i][j] = (dp[i][j] + dp[i - 1][j]) % modulo;
                }
                if j > 0 {
                    dp[i][j] = (dp[i][j] + dp[i][j - 1]) % modulo;
                }
            }
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}
