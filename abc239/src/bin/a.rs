use proconio::input;

fn main() {
    input! {
        h: f64,
    }

    println!("{}", (h * (h + 1.28e7)).sqrt());
}
