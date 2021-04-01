use proconio::input;

const MOD: usize = 2019;

fn main() {
    input! {
        s: String,
    }

    let mut cnt = vec![0usize; MOD];
    let mut ans = 0;
    for c in s.chars() {
        cnt[0] += 1;
        let c = (c as u8 - '0' as u8) as usize;
        let mut nxt = vec![0usize; MOD];
        for i in 0..MOD {
            nxt[(i * 10 + c) % MOD] += cnt[i];
        }
        ans += nxt[0];
        cnt = nxt;
    }

    println!("{}", ans);
}
