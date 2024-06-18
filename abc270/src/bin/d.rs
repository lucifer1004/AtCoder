use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }

    assert!(a[0] == 1); // This ensures the stones can always be fully taken
    let mut dp = vec![0; n + 1];

    for tot in 1..=n {
        for i in 0..k {
            if a[i] > tot {
                break;
            }

            dp[tot] = dp[tot].max(tot - dp[tot - a[i]]);
        }
    }

    println!("{}", dp[n]);
}
