use proconio::input;

fn main() {
    input! {
        k: usize,
        s: String,
    }

    println!("{}", if s.len() <= k {
        s
    } else {
        format!("{}{}", &s[0..k], "...")
    });
}
