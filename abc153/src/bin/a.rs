use proconio::input;

fn main() {
    input! {
        h: usize,
        a: usize,
    }

    println!("{}", (h - 1) / a + 1);
}
