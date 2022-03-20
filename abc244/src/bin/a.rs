use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    }

    println!("{}", s.chars().collect::<Vec<_>>().last().unwrap());
}
