use proconio::input;

const LOOKUP: [i64; 10] = [0, 1, 2, 3, 2, 1, 2, 3, 3, 2];

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let mut delta = (a - b).abs();
    println!("{}", delta / 10 + LOOKUP[(delta % 10) as usize])
}
