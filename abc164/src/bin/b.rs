use proconio::input;

fn main() {
    input! {
        mut a: i64,
        b: i64,
        mut c: i64,
        d: i64,
    }

    while a > 0 && c > 0 {
        c -= b;
        if c <= 0 {
            println!("Yes");
            std::process::exit(0);
        }
        a -= d;
        if a <= 0 {
            println!("No");
            std::process::exit(0);
        }
    }
}
