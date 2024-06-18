use proconio::input;

const MOD: usize = 998244353;

fn mod_exp(x: usize, mut y: usize) -> usize {
    if y == 0 {
        return 1;
    }

    let mut res = 1;
    let mut x = x % MOD;

    while y > 0 {
        if y % 2 == 1 {
            res = (res * x) % MOD;
        }

        y = y >> 1;
        x = (x * x) % MOD;
    }

    res
}

fn mod_inv(x: usize) -> usize {
    mod_exp(x, MOD - 2)
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut s = 0;
    let mut y = vec![0; n];
    for i in (1..n).rev() {
        let x = n * mod_inv(i) % MOD;
        let x = mod_exp(x, a[i] - a[i - 1]);
        let z = (n + MOD - s) % MOD * mod_inv(n - i) % MOD;
        y[i - 1] = ((y[i] + z) % MOD * x % MOD + MOD - z) % MOD;
        s = (s + y[i - 1]) % MOD;
    }

    println!("{}", y[0]);
}
