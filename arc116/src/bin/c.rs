use proconio::input;

const MOD: usize = 998_244_353;

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

    let mut is_prime = vec![true; m.max(2) + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= m {
        if is_prime[i] {
            let mut j = i * i;
            while j <= m {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let mut primes = vec![];
    for i in 2..=m.max(2) {
        if is_prime[i] {
            primes.push(i);
        }
    }

    let mut fac = vec![1usize; n + m + 1];
    let mut ifac = vec![1usize; n + m + 1];
    for i in 2..=n + m {
        fac[i] = fac[i - 1] * i % MOD;
        ifac[i] = inv(fac[i]);
    }

    let comb = |n: usize, k: usize| -> usize { fac[n] * ifac[k] % MOD * ifac[n - k] % MOD };

    let mut ans = 0;
    for num in 1..=m {
        let mut r = num;
        let mut j = 0;
        let mut fac_cnt = vec![];
        while primes[j] * primes[j] <= r {
            if r % primes[j] == 0 {
                let mut cnt = 0;
                while r % primes[j] == 0 {
                    r /= primes[j];
                    cnt += 1;
                }
                fac_cnt.push(cnt);
            }
            j += 1;
        }
        if r != 1 {
            fac_cnt.push(1);
        }
        let mut tot = 1;
        for cnt in fac_cnt {
            tot = tot * comb(n + cnt - 1, n - 1) % MOD;
        }
        ans = (ans + tot) % MOD;
    }

    println!("{}", ans);
}
