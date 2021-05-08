use proconio::input;

const MOD: usize = 1_000_000_007;

fn pow(mut x: usize, mut y: usize) -> usize {
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

fn main() {
    input! {
        s: String,
        k: usize,
    }

    let n = s.len();
    if n * k == 1 {
        println!("0");
        std::process::exit(0);
    }

    let inv2 = pow(2, MOD - 2);
    let s = s.chars().collect::<Vec<char>>();
    let mut q = 0;
    for i in 0..n {
        if s[i] == '?' {
            q += 1;
        }
    }

    let f = |a: usize, b: usize| -> usize {
        if s[a] == '?' || s[b] == '?' {
            inv2
        } else if s[a] == s[b] {
            0
        } else {
            1
        }
    };

    let mut v = 0;
    for i in 0..n {
        v = (v + f(i, (i + 1) % n)) % MOD;
    }
    v = v * k % MOD * pow(2, q * k) % MOD * inv2 % MOD;

    println!("{}", v);
}
