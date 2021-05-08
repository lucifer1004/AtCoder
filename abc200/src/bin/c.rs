use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut cnt = vec![0usize; 200];
    let mut ans = 0;
    for num in a {
        ans += cnt[num % 200];
        cnt[num % 200] += 1;
    }

    println!("{}", ans);
}
