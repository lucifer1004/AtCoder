use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut ans = n - 1;
    let mut i = 2u64;
    while i * i <= n {
        if n % i == 0 {
            ans = i + n / i - 2;
        }
        i += 1;
    }
    println!("{}", ans);
}
