use proconio::input;

const MOD: usize = 998_244_353;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    a.sort_unstable();
    let mut ans = 0usize;
    let mut sum = a[n - 1];
    for i in (0..n - 1).rev() {
        ans = (ans + sum * a[i]) % MOD;
        sum = (sum * 2 + a[i]) % MOD;
    }
    for i in 0..n {
        ans = (ans + a[i] * a[i]) % MOD;
    }

    println!("{}", ans);
}
