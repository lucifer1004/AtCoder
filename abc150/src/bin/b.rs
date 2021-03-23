use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans = 0;
    for i in 0..n - 2 {
        if s[i] == 'A' && s[i + 1] == 'B' && s[i + 2] == 'C' {
            ans += 1;
        }
    }

    println!("{}", ans);
}
