use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        h: f64,
        a: [(f64, f64); n],
    }

    let mut ans: f64 = 0.0;
    for (di, hi) in a {
        ans = ans.max(hi - (h - hi) / (d - di) * di);
    }

    println!("{}", ans);
}