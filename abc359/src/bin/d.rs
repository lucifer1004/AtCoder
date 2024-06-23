use proconio::input;

const MOD: usize = 998244353;

fn check_palindrome<T>(s: &Vec<T>, mut l: usize, mut r: usize) -> bool
where
    T: PartialEq,
{
    while l < r {
        if s[l] != s[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

fn main() {
    input! {
        n: usize,
        k: usize,
        s: String,
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut pre = vec![0; 1 << (k - 1)];
    for i in 0..1 << (k - 1) {
        // convert i to binary string
        let mut b = vec![0; k - 1];
        for j in 0..k - 1 {
            b[k - 2 - j] = (i >> j) & 1;
        }
        for nxt in 0..2 {
            b.push(nxt);
            if !check_palindrome(&b, 0, k - 1) {
                pre[i] |= 1 << nxt;
            }
            b.pop();
        }
    }

    let mut states = vec![0];
    let mut dp = vec![0; 1 << (k - 1)];
    for i in 0..n {
        if i < k - 1 {
            let mut nxt = vec![];
            for &state in &states {
                let nstate = state << 1;
                if s[i] == 'A' || s[i] == '?' {
                    nxt.push(nstate);
                }
                if s[i] == 'B' || s[i] == '?' {
                    nxt.push(nstate | 1);
                }
            }
            states = nxt;

            if i == k - 2 {
                for &state in &states {
                    dp[state] = 1;
                }
            }
        } else {
            let mut ndp = vec![0; 1 << (k - 1)];
            for state in 0..1 << (k - 1) {
                if dp[state] == 0 {
                    continue;
                }
                let mut j = 0;
                if s[i] == 'A' || s[i] == '?' {
                    if pre[state] & (1 << j) != 0 {
                        let nstate = ((state << 1) | j) & ((1 << (k - 1)) - 1);
                        ndp[nstate] += dp[state];
                        ndp[nstate] %= MOD;
                    }
                }
                j = 1;
                if s[i] == 'B' || s[i] == '?' {
                    if pre[state] & (1 << j) != 0 {
                        let nstate = ((state << 1) | j) & ((1 << (k - 1)) - 1);
                        ndp[nstate] += dp[state];
                        ndp[nstate] %= MOD;
                    }
                }
            }
            dp = ndp;
        }
    }

    println!("{}", dp.iter().sum::<usize>() % MOD);
}
