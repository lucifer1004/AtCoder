use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        weight_limit: usize,
        items: [(usize, usize); n],
    }

    let mut sum = 0usize;
    for (_, value) in items.clone() {
        sum += value;
    }
    let mut dp = vec![weight_limit + 1; sum + 1];
    dp[0] = 0;
    for (weight, value) in items.clone() {
        for v in (0..=sum).rev() {
            if dp[v] + weight <= weight_limit {
                dp[v + value] = min(dp[v + value], dp[v] + weight);
            }
        }
    }
    for v in (0..=sum).rev() {
        if dp[v] <= weight_limit {
            println!("{}", v);
            return;
        }
    }
}
