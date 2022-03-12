use proconio::input;

fn main() {
    input! {
        v: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    let rem = v % (a + b + c);
    if rem < a {
        println!("F");
    } else if rem < a + b {
        println!("M");
    } else {
        println!("T");
    }
}
