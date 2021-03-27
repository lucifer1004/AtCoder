use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 1 << 30;
    for i in 1..(1 << n) {
        let mut xor = 0;
        let mut or = 0;
        for j in 0..n {
            or |= a[j];
            if i & (1 << j) > 0 {
                xor ^= or;
                or = 0;
            }
        }
        xor ^= or;
        ans = ans.min(xor);
    }

    println!("{}", ans);
}
