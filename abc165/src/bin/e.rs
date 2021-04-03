use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if n % 2 == 1 {
        for i in 1..=m {
            println!("{} {}", i, n - i);
        }
    } else {
        let mut i = 1;
        while i <= m {
            if n + 1 - i * 2 > n / 2 {
                println!("{} {}", i, n + 1 - i);
            } else {
                println!("{} {}", i, n - i);
            }
            i += 1;
        }
    }
}
