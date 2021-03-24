use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        mut h: [usize; n],
    }

    k = k.min(n);
    h.sort();
    let mut sum = 0usize;
    for i in 0..n - k {
        sum += h[i];
    }

    println!("{}", sum);
}
