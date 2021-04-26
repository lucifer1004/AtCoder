use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [(usize, usize, usize); m],
    }

    let mut dp = vec![0usize; 1 << n];
    dp[0] = 1;
    let mut val = (0..1 << n).into_iter().collect::<Vec<usize>>();
    val.sort_unstable_by_key(|a| {
        a.count_ones()
    });

    for state in val {
        if dp[state] == 0 {
            continue;
        }
        let ones = state.count_ones() as usize;
        let mut cnt = vec![0; n];
        for i in 0..n {
            if state & (1 << i) > 0 {
                cnt[i] += 1;
            }
        }
        for i in 0..n {
            if state & (1 << i) == 0 {
                cnt[i] += 1;
                let mut acc = vec![0; n + 1];
                for j in 1..=n {
                    acc[j] = acc[j - 1] + cnt[j - 1];
                }
                let mut ok = true;
                for j in 0..m {
                    if c[j].0 != ones + 1 {
                        continue;
                    }
                    if acc[c[j].1] > c[j].2 {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    dp[state ^ (1 << i)] += dp[state];
                }
                cnt[i] -= 1;
            }
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}
