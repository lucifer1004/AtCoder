use proconio::input;
use std::cmp::min;
use std::usize::MAX;

fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let mut dp = vec![MAX; 1 << n];
    dp[0] = 0;

    for _ in 0..m {
        input! {
            (a, b): (usize, usize),
            c: [usize; b],
        }

        let mut state = 0usize;
        for ci in c {
            state ^= 1 << (ci - 1);
        }

        for i in 0usize..(1 << n) {
            if dp[i] < MAX {
                dp[i | state] = min(dp[i | state], dp[i] + a);
            }
        }
    }

    if dp[(1 << n) - 1] == MAX {
        println!("-1");
    } else {
        println!("{}", dp[(1 << n) - 1]);
    }
}
