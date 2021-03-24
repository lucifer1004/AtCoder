use proconio::input;

const INF: usize = 1_000_000_000_000_000_000;

fn main() {
    input! {
        h: usize,
        n: usize,
        spells: [(usize, usize); n],
    }

    let mut dp = vec![INF; h + 1];
    dp[0] = 0;
    for (damage, cost) in spells {
        for i in 0..h {
            let nxt = (i + damage).min(h);
            dp[nxt] = dp[nxt].min(dp[i] + cost);
        }
    }

    println!("{}", dp[h]);
}
