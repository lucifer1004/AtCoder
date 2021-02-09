use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    if n % 2 != 0 {
        println!("No");
    } else {
        for i in 0..n / 2 {
            if s[i] != s[n / 2 + i] {
                println!("No");
                return;
            }
        }
        println!("Yes");
    }
}
