use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        t: [usize; n],
    }

    let mut now = 0;
    for ti in t {
        now = now.max(ti) + a;
        print!("{} ", now);
    }
}