use proconio::input;

fn main() {
    input! {
        n: usize,
        c: String,
    }

    let mut cnt = vec![0; 5];

    for ch in c.chars() {
        cnt[str::parse::<usize>(&ch.to_string()).unwrap()] += 1;
    }

    let mut lo = n;
    let mut hi = 0;

    for i in 1..=4 {
        lo = lo.min(cnt[i]);
        hi = hi.max(cnt[i]);
    }

    println!("{} {}", hi, lo);
}
