use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn mod_exp(x: usize, mut y: usize, p: usize) -> usize {
    if y == 0 {
        return 1;
    }

    let mut res = 1;
    let mut x = x % p;

    while y > 0 {
        if y % 2 == 1 {
            res = (res * x) % p;
        }

        y = y >> 1;
        x = (x * x) % p;
    }

    res
}

fn mod_inv(x: usize, p: usize) -> usize {
    mod_exp(x, p - 2, p)
}

// Use Baby-step Giant-step algorithm to solve the discrete logarithm problem: a^x = b (mod p)
fn baby_step_giant_step(a: usize, b: usize, p: usize) -> usize {
    assert_eq!(gcd(a, p), 1); // a and p must be coprime for the algorithm to work

    let a = a % p;
    let b = b % p;
    let m = (p as f64).sqrt().ceil() as usize;
    let am = mod_exp(a, m, p);

    // Baby-step
    let mut table = std::collections::HashMap::new();
    let mut cur = b;
    for i in 0..m {
        table.insert(cur, i);
        cur = cur * a % p;
    }

    // Giant-step
    cur = 1;
    for i in 1..=m {
        cur = cur * am % p;
        if let Some(j) = table.get(&cur) {
            return i * m - j;
        }
    }

    return std::usize::MAX;
}

fn main() {
    input! {
        t: usize,
        cases: [(usize, usize, usize, usize, usize); t],
    }

    // X0 = s, Xi = (a * Xi-1 + b) % p
    for (p, a, b, s, g) in cases {
        // Special case 0: s = g
        if s == g {
            println!("0");
            continue;
        }

        if a == 0 {
            // Special case 1: a = 0
            if b == g {
                println!("1");
            } else {
                println!("-1");
            }
        } else if a == 1 {
            // Special case 2: a = 1
            if b != 0 {
                println!("{}", (g + p - s) % p * mod_inv(b, p) % p);
            } else {
                println!("-1");
            }
        } else {
            let nom = (g * (a - 1) + b) % p;
            let denom = (s * (a - 1) + b) % p;
            if nom != 0 && denom != 0 {
                let i = baby_step_giant_step(a, nom * mod_inv(denom, p) % p, p);
                if i == std::usize::MAX {
                    println!("-1");
                } else {
                    println!("{}", i);
                }
            } else {
                println!("-1");
            }
        }
    }
}
