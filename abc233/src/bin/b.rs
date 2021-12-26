use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
        s: String,
    }

    let left = &s[..l - 1];
    let mut mid = s[l - 1..r].to_string().chars().collect::<Vec<_>>();
    mid.reverse();
    let mid = mid.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join("");
    let right = &s[r..];

    println!("{}{}{}", left, mid, right);
}
