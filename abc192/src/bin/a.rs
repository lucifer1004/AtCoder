use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    println!("{}", 100 - x % 100);
}
