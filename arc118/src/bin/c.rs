#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    if n == 3 {
        println!("6 10 15");
    } else {
        let mut v = vec![];
        let mut a = 6;
        let mut b = 10;
        let mut c = 15;

        for _ in 0..n {
            let m = a.min(b).min(c);
            if a == m {
                a += 6;
            }
            if b == m {
                b += 10;
            }
            if c == m {
                c += 15;
            }
            v.push(m);
        }

        println!(
            "{}",
            v.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
