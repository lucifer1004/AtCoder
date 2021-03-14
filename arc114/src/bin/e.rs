use proconio::input;

const MOD: usize = 998_244_353;

fn fast_exp(mut x: usize, mut y: usize) -> usize {
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

fn inv(x: usize) -> usize {
    fast_exp(x, MOD - 2)
}

fn main() {
    input! {
        h: usize,
        w: usize,
        h1: usize,
        w1: usize,
        h2: usize,
        w2: usize,
    }

    let l = w1.min(w2);
    let r = w1.max(w2);
    let u = h1.min(h2);
    let d = h1.max(h2);
    let l_left = l - 1;
    let r_left = w - r;
    let u_left = u - 1;
    let d_left = h - d;
    let inner = r - l + d - u;

    // The four sides are independent from each other,
    // since when we cut from one side, the other sides are not affected!
    let mut ans = 1;
    for i in 1..=l_left {
        ans += inv(inner + i);
        ans %= MOD;
    }
    for i in 1..=r_left {
        ans += inv(inner + i);
        ans %= MOD;
    }
    for i in 1..=u_left {
        ans += inv(inner + i);
        ans %= MOD;
    }
    for i in 1..=d_left {
        ans += inv(inner + i);
        ans %= MOD;
    }

    println!("{}", ans);
}