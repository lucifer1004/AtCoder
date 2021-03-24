use proconio::input;

fn kill(h: usize) -> usize {
    if h == 1 {
        1
    } else {
        kill(h / 2) * 2 + 1
    }
}

fn main() {
    input! {
        h: usize,
    }

    println!("{}", kill(h));
}
