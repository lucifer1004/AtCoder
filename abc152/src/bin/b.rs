use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let lo = a.min(b);
    let hi = a.max(b);
    println!("{}", vec![lo.to_string(); hi].concat());
}
