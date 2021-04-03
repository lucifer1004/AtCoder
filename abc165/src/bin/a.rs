use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    }

    if b - a + 1 >= k || a % k > b % k || a % k == 0 {
        println!("OK");
    } else {
        println!("NG");
    }
}
