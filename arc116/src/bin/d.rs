use proconio::input;

const MOD: usize = 998_244_353;
const N: usize = 10_000;

fn fexp(mut x: usize, mut y: usize) -> usize {
    let mut ans = 1;
    while y > 0 {
        if y & 1 == 1 {
            ans = ans * x % MOD;
        }
        x = x * x % MOD;
        y >>= 1;
    }
    ans
}

fn inv(x: usize) -> usize {
    fexp(x, MOD - 2)
}

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    if m % 2 == 1 || n == 1 {
        println!("0");
        std::process::exit(0);
    }

    let mut fac = vec![1usize; N + 1];
    let mut ifac = vec![1usize; N + 1];
    for i in 2..=N {
        fac[i] = fac[i - 1] * i % MOD;
        ifac[i] = inv(fac[i]);
    }

    let comb = |n: usize, k: usize| -> usize { fac[n] * ifac[k] % MOD * ifac[n - k] % MOD };

    let mut dp = vec![0usize; m + 1];
    dp[0] = 1;
    for i in (0usize..=12).rev() {
        if m & (1 << i) > 0 {
            for j in (1..=m).rev() {
                dp[j] = dp[j - 1];
            }
            dp[0] = 0;
        }
        let mut ndp = vec![0usize; m + 1];
        for j in 0..=m {
            if j * 2 <= m {
                ndp[j * 2] = (ndp[j * 2] + dp[j]) % MOD;
            }
            for k in (2..=n).step_by(2) {
                if k > j {
                    break;
                }
                if (j - k) * 2 <= m {
                    ndp[(j - k) * 2] = (ndp[(j - k) * 2] + dp[j] * comb(n, k)) % MOD;
                }
            }
        }
        dp = ndp;
    }

    println!("{}", dp[0]);
}
