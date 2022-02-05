use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    for i in 0..w {
        for j in 0..h {
            print!("{} ", a[j][i]);
        }
        println!();
    }
}
