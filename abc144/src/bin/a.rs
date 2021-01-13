use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    println!("{}", if std::cmp::max(a, b) <= 9 { a * b } else { -1 });
}
