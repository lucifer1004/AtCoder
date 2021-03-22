use proconio::input;

fn main() {
    input! {
        wrong: [usize; 2]
    }

    println!("{}", 6 - wrong[0] - wrong[1]);
}
