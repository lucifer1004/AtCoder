use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut now = 100;
    let mut year = 0;
    while now < x {
        now += now / 100;
        year += 1;
    }

    println!("{}", year);
}
