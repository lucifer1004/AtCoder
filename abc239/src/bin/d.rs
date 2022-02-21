use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let mut is_prime = vec![true; b + d + 1];
    is_prime[1] = false;
    let mut primes = vec![];
    for i in 2..=b + d {
        if is_prime[i] {
            primes.push(i);
        }

        for &prime in primes.iter() {
            if i * prime > b + d {
                break;
            }
            is_prime[i * prime] = false;
            if i % prime == 0 {
                break;
            }
        }
    }

    let mut takahashi_win = false;
    for takahashi in a..=b {
        let mut aoki_win = false;
        for aoki in c..=d {
            if is_prime[takahashi + aoki] {
                aoki_win = true;
                break;
            }
        }
        if !aoki_win {
            takahashi_win = true;
            break;
        }
    }

    if takahashi_win {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
