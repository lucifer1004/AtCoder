use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort();
    b.sort();

    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;

    while i < n && j < m {
        if a[i] >= b[j] {
            ans += a[i];
            i += 1;
            j += 1;
        } else {
            i += 1;
        }
    }

    if j == m {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
