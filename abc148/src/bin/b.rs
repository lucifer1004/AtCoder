use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    println!(
        "{}",
        (0..n)
            .map(|i| {
                let v = vec![s[i].to_string(), t[i].to_string()];
                v.join("")
            })
            .collect::<Vec<String>>()
            .join("")
    );
}
