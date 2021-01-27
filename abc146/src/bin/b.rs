use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
        mut s: Bytes,
    }

    for (i, c) in s.clone().into_iter().enumerate() {
        s[i] = (c - b'A' + n as u8) % 26 + b'A';
    }

    println!("{}", s.into_iter().map(|x| x as char).collect::<String>());
}
