use proconio::input;

fn main() {
    input! {
        a: [i64; 4],
    }

    println!("{}", (a[0] + a[1]) * (a[2] - a[3]));
    println!("Takahashi");
}
