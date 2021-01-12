use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    println!("{}", ((n + 1) / 2) as f32 / n as f32);
}
