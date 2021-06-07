use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    if x == y {
        println!("{}", x);
    } else {
        println!("{}", 3 - x - y);
    }
}
