use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [usize; n],
    }

    l.sort();
    let mut ans = 0usize;
    for i in 0..n - 2 {
        let mut k = i + 2;
        for j in i + 1..n - 1 {
            if k <= j {
                k = j + 1;
            }
            while k + 1 < n && l[k + 1] < l[i] + l[j] {
                k += 1;
            }
            if l[k] < l[i] + l[j] {
                ans += k - j;
            }
        }
    }

    println!("{}", ans);
}
