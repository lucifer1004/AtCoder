use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut dp = vec![false; k + 1];
    for i in 1..=k {
        for ai in a.clone() {
            if i < ai {
                break;
            }
            if !dp[i - ai] {
                dp[i] = true;
                break;
            }
        }
    }

    println!("{}", if dp[k] { "First" } else { "Second" });
}
