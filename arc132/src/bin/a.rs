use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        r: [usize; n],
        c: [usize; n],
        q: usize,
        queries: [(Usize1, Usize1); q],
    }

    println!("{}", queries.into_iter().map(|(i, j)| if r[i] + c[j] > n { "#" } else { "." }).collect::<Vec<_>>().join(""));
}
