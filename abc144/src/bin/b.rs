use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    for i in 1..=std::cmp::min(9, n) {
        if n % i == 0 && n / i <= 9 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
