use proconio::input;

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    println!("{}", a * b / gcd(a, b));
}
