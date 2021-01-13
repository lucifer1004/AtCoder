use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        mut a: [i64; n],
        mut f: [i64; n],
    }

    a.sort();
    f.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut lo = 0i64;
    let mut hi = 1_000_000_000_000i64;
    while lo <= hi {
        let mid = (lo + hi) >> 1;
        let mut tot = 0i64;
        for i in 0usize..n {
            tot += std::cmp::max(a[i] - mid / f[i], 0);
        }
        if tot <= k {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    println!("{}", lo);
}
