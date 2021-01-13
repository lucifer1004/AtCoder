use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let n = s.len();
    let m = t.len();

    let mut dp = vec![vec![0; m]; n];
    let mut prev = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            if s[i] == t[j] {
                dp[i][j] = if i > 0 && j > 0 {
                    dp[i - 1][j - 1] + 1
                } else {
                    1
                };
            } else {
                if i > 0 {
                    dp[i][j] = dp[i - 1][j];
                    prev[i][j] = -1;
                }
                if j > 0 && dp[i][j - 1] > dp[i][j] {
                    dp[i][j] = dp[i][j - 1];
                    prev[i][j] = 1;
                }
            }
        }
    }

    let mut cnt = dp[n - 1][m - 1];
    let mut ans: Vec<String> = vec![];
    let mut x: i32 = n as i32 - 1;
    let mut y: i32 = m as i32 - 1;
    while cnt > 0 {
        match prev[x as usize][y as usize] {
            0 => {
                ans.push(s[x as usize].to_string());
                x -= 1;
                y -= 1;
                cnt -= 1;
            }
            -1 => {
                x -= 1;
            }
            _ => {
                y -= 1;
            }
        }
    }

    ans.reverse();
    println!("{}", ans.join(""));
}
