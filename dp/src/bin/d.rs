use proconio::input;

fn main() {
    input! {
        n: usize,
        weight_limit: usize,
        items: [(usize, u64); n],
    }

    let mut dp = vec![0u64; weight_limit + 1];
    for (weight, value) in items {
        for j in (weight..=weight_limit).rev() {
            dp[j] = std::cmp::max(dp[j], dp[j - weight] + value);
        }
    }

    println!("{}", dp[weight_limit]);
}
