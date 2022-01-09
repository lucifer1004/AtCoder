use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let s = format!("{:b}", k).replace("1", "2");
    println!("{}", s);
}
