use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut hard = true;
    for i in 0..s.len() {
        if (i % 2 == 0 && s[i] == s[i].to_ascii_uppercase()) || (i % 2 == 1 && s[i] == s[i].to_ascii_lowercase()) {
            hard = false;
            break;
        }
    }
    println!("{}", if hard {"Yes"} else {"No"});
}
