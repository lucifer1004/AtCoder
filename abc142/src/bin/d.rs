use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn sieve(upper_bound: usize) -> Vec<u64> {
    let mut mark = vec![true; upper_bound + 1];
    mark[0] = false;
    mark[1] = false;
    for i in 2..=upper_bound {
        if mark[i] {
            for j in (i * 2..=upper_bound).step_by(i) {
                mark[j] = false;
            }
        }
    }
    mark.into_iter()
        .enumerate()
        .filter(|x| x.1)
        .map(|x| x.0 as u64)
        .collect()
}

fn main() {
    input! {
        a: u64,
        b: u64
    }

    let mut g = gcd(a, b);

    let primes = sieve((g as f64).sqrt() as usize + 1);

    let mut ans = 1;

    for prime in primes {
        if g < prime {
            break;
        }
        if g % prime == 0 {
            ans += 1;
            while g % prime == 0 {
                g /= prime;
            }
        }
    }

    if g > 1 {
        ans += 1;
    }

    println!("{}", ans);
}
