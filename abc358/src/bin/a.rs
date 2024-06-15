use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    if s == "AtCoder" && t == "Land" {
        println!("Yes");
    } else {
        println!("No");
    }
}