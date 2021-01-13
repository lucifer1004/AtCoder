use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        (a, b): (i32, i32)
    }
    println!("{}", max(0, a - 2 * b));
}
