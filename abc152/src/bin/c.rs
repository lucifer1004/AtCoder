use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut ans = 0;
    let mut lo = n + 1;
    for pi in p {
        lo = lo.min(pi);
        if lo == pi {
            ans += 1;
        }
    }

    println!("{}", ans);
}
