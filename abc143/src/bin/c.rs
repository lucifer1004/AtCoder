use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let s: Vec<char> = s.chars().into_iter().collect();
    let mut ans = 1;
    for i in 1..n {
        if s[i] != s[i - 1] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
