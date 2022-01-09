use proconio::input;

const MOD: usize = 998_244_353;

fn mod_exp(mut x: usize, mut y: usize) -> usize {
    let mut ans = 1;
    while y > 0 {
        if (y & 1) == 1 {
            ans = ans * x % MOD;
        }
        x = x * x % MOD;
        y >>= 1;
    }
    ans
}

fn mod_inv(x: usize) -> usize {
    mod_exp(x, MOD - 2)
}

fn main() {
    input! {
        s: String,
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut cnt = vec![0; 26];
    for &ch in s.iter() {
        cnt[(ch as u8 as usize) - ('a' as u8 as usize)] += 1;
    }

    let n = s.len();
    let mut fac = vec![1usize; n + 1];
    let mut ifac = vec![1usize; n + 1];
    for i in 2..=n {
        fac[i] = fac[i - 1] * i % MOD;
    }
    ifac[n] = mod_inv(fac[n]);
    for i in (2..n).rev() {
        ifac[i] = ifac[i + 1] * (i + 1) % MOD;
    }

    let mut dp = vec![0usize; n + 1];
    dp[0] = 1;
    let mut acc = 0;
    for i in 0..26 {
        if cnt[i] == 0 {
            continue;
        }

        for j in (0..=acc).rev() {
            for k in (1..=cnt[i]).rev() {
                dp[j + k] = (dp[j + k] + dp[j] * fac[j + k] % MOD * ifac[j] % MOD * ifac[k] % MOD) % MOD;
            }
        }

        acc += cnt[i];
    }

    let mut ans = 0;
    for i in 1..=n {
        ans = (ans + dp[i]) % MOD;
    }

    println!("{}", ans);
}
