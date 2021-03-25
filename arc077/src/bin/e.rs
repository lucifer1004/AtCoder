use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }

    let mut jump = vec![0i64; m + 1];
    let mut dec_delta = vec![0i64; m + 1];

    let m = m as i64;
    let mut now = 0i64;
    for i in 0..n - 1 {
        let target = if a[i + 1] > a[i] {
            a[i + 1]
        } else {
            a[i + 1] + m
        };
        let delta = target - a[i];
        now += delta.min(a[i + 1]);

        let left = target - delta + 1;
        jump[(target % m + 1) as usize] += delta - 1;
        if target <= m {
            dec_delta[((left - 1) % m + 1) as usize] += 1;
            dec_delta[((target - 1) % m + 1) as usize] -= 1;
        } else {
            dec_delta[1] += 1;
            dec_delta[((target - 1) % m + 1) as usize] -= 1;
            if (left - 1) % m + 1 > 1 {
                dec_delta[((left - 1) % m + 1) as usize] += 1;
            }
        }
    }

    let mut ans = now;
    let mut dec = dec_delta[1];

    let m = m as usize;
    for i in 2..=m {
        now += jump[i] - dec;
        ans = ans.min(now);
        dec += dec_delta[i];
    }

    println!("{}", ans);
}
