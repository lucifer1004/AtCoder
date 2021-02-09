use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let mut ans = 0;
    for i in 0..n / 2 {
        if s[i] != s[n - 1 - i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
