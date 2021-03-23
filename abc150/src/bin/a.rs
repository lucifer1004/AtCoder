use proconio::input;

fn main() {
    input! {
        k: usize,
        x: usize,
    }

    println!("{}", if k * 500 >= x { "Yes" } else { "No" });
}
