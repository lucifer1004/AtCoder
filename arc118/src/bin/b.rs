#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
        n: usize,
        m: usize,
        a: [usize; k],
    }

    let mut b = a.clone().iter().map(|x| x * m / n).collect::<Vec<usize>>();
    let mut rem = m;
    let mut v = vec![];
    for i in 0..k {
        rem -= b[i];
        if b[i] * n < a[i] * m {
            v.push((a[i] * m - b[i] * n, i));
        }
    }
    v.sort_by_key(|x| std::u32::MAX as usize - x.0);

    for i in 0..rem {
        b[v[i].1] += 1;
    }

    println!(
        "{}",
        b.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
