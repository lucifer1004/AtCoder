use proconio::input;

fn main() {
    input! {
        h: usize,
        n: usize,
        a: [usize; n],
    }

    let mut sum = a.into_iter().fold(0, |a, c| a + c);
    println!("{}", if sum >= h { "Yes" } else { "No" });
}
