use proconio::input;

const N: usize = 200005;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut invalid = 0usize;
    let mut cnt = vec![0usize; N];
    for ai in a {
        cnt[ai] += 1;
    }
    for i in 1..N {
        if cnt[i] >= 2 {
            invalid += cnt[i] * (cnt[i] - 1) / 2 * (n - cnt[i]);
        }
        if cnt[i] >= 3 {
            invalid += cnt[i] * (cnt[i] - 1) * (cnt[i] - 2) / 6;
        }
    }

    println!("{}", n * (n - 1) * (n - 2) / 6 - invalid);
}
