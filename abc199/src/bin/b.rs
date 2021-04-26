use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut lo = 1;
    let mut hi = 1000;
    for i in 0..n {
        lo = lo.max(a[i]);
        hi = hi.min(b[i]);
    }

    println!("{}", if lo > hi {
        0
    } else {
        hi - lo + 1
    })
}
