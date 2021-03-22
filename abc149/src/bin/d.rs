use std::collections::HashMap;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        points: [usize; 3],
        t: Chars,
    }

    let mut mp = HashMap::new();
    mp.insert('r', 0);
    mp.insert('s', 1);
    mp.insert('p', 2);

    let mut ans = 0;

    for i in 0..k {
        let mut dp = vec![0; 3];
        for j in (i..n).step_by(k) {
            let mut ndp = vec![0; 3];
            let bot = *mp.get(&t[j]).unwrap();
            for player in 0..3 {
                let extra = if (player + 1) % 3 == bot {
                    points[player]
                } else {
                    0
                };
                ndp[player] = dp[(player + 1) % 3].max(dp[(player + 2) % 3]) + extra;
            }
            dp = ndp;
        }
        ans += dp[0].max(dp[1]).max(dp[2]);
    }

    println!("{}", ans);
}
