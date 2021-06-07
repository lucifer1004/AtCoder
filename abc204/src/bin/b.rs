use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for ai in a {
        ans += if ai >= 10 { ai - 10 } else { 0 };
    }

    println!("{}", ans);
}
