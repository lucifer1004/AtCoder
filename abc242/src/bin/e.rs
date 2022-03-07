use proconio::input;

const MOD: usize = 998_244_353;

fn main() {
    input! {
        t: usize,
        cases: [(usize, String); t],
    }

    for (n, s) in cases {
        let s = s.chars()
            .map(|x| (x as u8 - 'A' as u8) as usize)
            .collect::<Vec<_>>();

        let mut ans = 0;
        let mut same = s.clone();
        for i in n / 2..n {
            same[i] = same[n - 1 - i];
        }

        if same <= s {
            ans += 1;
        }

        let mut small = 0;
        for i in 0..=(n - 1) / 2 {
            small = small * 26 + s[i];
            small %= MOD;
        }

        ans = (ans + small) % MOD;
        println!("{}", ans);
    }
}
