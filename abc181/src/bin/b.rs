use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut ans = 0u64;
    for _ in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        ans += (a + b) * (b - a + 1) / 2;
    }
    println!("{}", ans);
}
