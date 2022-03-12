use proconio::input;

const N: usize = 60000;

fn sqrt(x: usize) -> usize {
    let mut lo = 1_u128;
    let mut hi = x as u128;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if mid * mid <= x as u128 {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    hi as usize
}

fn main() {
    input! {
        t: usize,
        x: [usize; t],
    }

    let mut d = vec![0; N + 1];
    let mut dsum = vec![0; N + 1];
    d[1] = 1;
    dsum[1] = 1;
    for i in 2..=N {
        d[i] = dsum[sqrt(i)];
        dsum[i] = dsum[i - 1] + d[i];
    }

    for xi in x {
        if xi <= N {
            println!("{}", d[xi]);
        } else {
            let mut ans = 0;
            let ub = sqrt(xi);

            for i in 1..=N {
                let lo = i * i;
                if lo > ub {
                    break;
                }
                let hi = ((i + 1) * (i + 1) - 1).min(ub);
                ans += dsum[i] * (hi - lo + 1);
            }

            println!("{}", ans);
        }
    }
}
