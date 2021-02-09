use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        mut dishes: [(usize, usize); n],
    }

    dishes.sort_by(|a, b| {
        if a.0 < b.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut ans = 0;
    let mut dp = vec![0; t];

    for (a, b) in dishes {
        for i in 0..t {
            ans = ans.max(dp[i] + b);
        }
        for i in (t.min(a)..t).rev() {
            dp[i] = dp[i].max(dp[i - a] + b);
        }
    }

    println!("{}", ans);
}
