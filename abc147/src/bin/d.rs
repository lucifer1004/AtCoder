use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0usize;
    let mut two = 1usize;
    for i in 0..60 {
        let mut set = 0;
        let mask = 1usize << i;
        if a[n - 1] & mask > 0 {
            set += 1;
        }
        for j in (0..n - 1).rev() {
            if a[j] & mask > 0 {
                ans = (ans + two * (n - j - 1 - set)) % MOD;
                set += 1;
            } else {
                ans = (ans + two * set) % MOD;
            }
        }
        two = two * 2 % MOD;
    }

    println!("{}", ans);
}
