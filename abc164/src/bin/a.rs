use proconio::input;

fn main() {
    input! {
        s: usize,
        w: usize,
    }

    println!("{}", if w >= s { "unsafe" } else { "safe" });
}
