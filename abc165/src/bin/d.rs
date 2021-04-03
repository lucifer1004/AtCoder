use proconio::input;

fn f(a: usize, b: usize, x: usize) -> usize {
    a * x / b - a * (x / b)
}

fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize,
    }

    let mut ans = f(a, b, n);
    if n >= b {
        ans = ans.max(f(a, b, n / b * b - 1));
    }

    println!("{}", ans);
}
