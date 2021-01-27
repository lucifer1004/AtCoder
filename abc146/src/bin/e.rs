use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    }

    if k == 1 {
        println!("0");
        return;
    }

    let mut ans = 0usize;
    let mut sum: Vec<usize> = vec![0usize; n + 1];
    let mut cnt = HashMap::new();
    cnt.insert(0usize, 1usize);

    for (i, a_i) in a.into_iter().enumerate() {
        sum[i + 1] = (sum[i] + a_i) % k;
        match cnt.get(&sum[i + 1]) {
            Some(&freq) => {
                ans += freq;
                cnt.insert(sum[i + 1], freq + 1);
            }
            None => {
                cnt.insert(sum[i + 1], 1);
            }
        }
        if i >= k - 2 {
            cnt.insert(sum[i + 2 - k], cnt.get(&sum[i + 2 - k]).unwrap() - 1);
        }
    }

    println!("{}", ans);
}
