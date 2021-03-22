use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    if n % 2 == 1 {
        println!("0");
    } else {
        n /= 2;
        let mut ans = 0;
        while n > 0 {
            ans += n / 5;
            n /= 5;
        }
        println!("{}", ans);
    }
}
