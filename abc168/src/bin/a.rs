use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", match n % 10 {
        3 => "bon",
        0 | 1 | 6 | 8 => "pon",
        _ => "hon",
    });
}
