use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }

    let (ra, rb) = if k < a {
        (a - k, b)
    } else if k < a + b {
        (0, a + b - k)
    } else {
        (0, 0)
    };

    println!("{} {}", ra, rb);
}
