use proconio::input;

fn main() {
    input! {
        mut a: usize,
        b: usize,
        k: usize,
    }

    let mut ans = 0;
    while a < b {
        a *= k;
        ans += 1;
    }

    println!("{}", ans);
}
