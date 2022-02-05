use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    println!("{}", if x >= -(1i64 << 31) && x < (1i64 << 31) { "Yes" } else { "No" });
}
