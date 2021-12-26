use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut i = 0;
    while p[i] != 1 {
        i += 1;
    }

    let pre = p[(i + n - 1) % n];
    let ans = if pre == n {
        i.min(2 + n - i)
    } else {
        (i + 2).min(n - i)
    };

    println!("{}", ans);
}
