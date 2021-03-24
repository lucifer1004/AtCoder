use proconio::input;

const MOD: usize = 1_000_000_007;

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
        mut a: [usize; n],
    };

    let hi = *a.clone().iter().max().unwrap();

    let mut is_prime = vec![true; hi + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut base = 2;
    let mut primes = vec![];
    while base * base <= hi {
        if is_prime[base] {
            primes.push(base);
            for j in (base * base..=hi).step_by(base) {
                is_prime[j] = false;
            }
        }
        base += 1;
    }

    let mut counter = vec![0; hi + 1];
    let mut factors = vec![vec![]; n];
    for i in 0..n {
        for prime in primes.clone() {
            if prime * prime > a[i] {
                break;
            }
            if a[i] % prime == 0 {
                let mut cnt = 0;
                while a[i] % prime == 0 {
                    a[i] /= prime;
                    cnt += 1;
                }
                counter[prime] = counter[prime].max(cnt);
                factors[i].push((prime, cnt));
            }
        }
        if a[i] > 1 {
            counter[a[i]] = counter[a[i]].max(1);
            factors[i].push((a[i], 1));
        }
    }

    let mut mul = 1;
    for i in 2..=hi {
        if counter[i] > 0 {
            mul = mul * fexp(i, counter[i]) % MOD;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        let mut remove = 1;
        for (prime, cnt) in factors[i].clone() {
            remove = remove * fexp(prime, cnt) % MOD;
        }
        ans = (ans + mul * inv(remove)) % MOD;
    }

    println!("{}", ans);
}
