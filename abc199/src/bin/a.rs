use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    println!("{}", if a * a + b * b < c * c {
        "Yes"
    } else {
        "No"
    })
}
