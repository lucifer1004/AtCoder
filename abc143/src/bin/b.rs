use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            ans += a[i] * a[j];
        }
    }

    println!("{}", ans);
}
