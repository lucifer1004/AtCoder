use proconio::input;
use std::collections::HashMap;

const INF: i32 = 1_000_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        cards: [(i32, i32); n],
    }

    let mut delta: HashMap<i32, usize> = HashMap::new();
    let mut sum: usize = 0;
    for &(a, b) in cards.iter() {
        sum += a as usize;
        *delta.entry(b - a).or_insert(0) += 1;
    }

    let mut dp = vec![INF; m + 1];
    dp[sum] = 0;
    let mut items: Vec<(i32, i32)> = vec![];
    for (&k, &v) in delta.iter() {
        let mut i = v;
        let mut j = 1;
        while i >= j {
            items.push((k * j as i32, j as i32));
            i -= j;
            j *= 2;
        }
        if i > 0 {
            items.push((k * i as i32, i as i32));
        }
    }

    for &(k, v) in items.iter() {
        let mut ndp = dp.clone();
        for i in 0..=m {
            if dp[i] == INF || (i as i32 + k) < 0 || (i as i32 + k) as usize > m {
                continue;
            }

            let nxt = (i as i32 + k) as usize;
            ndp[nxt] = ndp[nxt].min(dp[i] + v);
        }
        dp = ndp;
    }

    for i in 0..=m {
        println!("{}", if dp[i] == INF { -1 } else { dp[i] });
    }
}
