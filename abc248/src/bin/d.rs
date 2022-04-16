use proconio::input;

fn query(a: &[usize], b: usize) -> usize {
    a.binary_search(&b).unwrap_or_else(|x| x)
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        queries: [(usize, usize, usize); q],
    }

    let mut pos = vec![vec![]; n + 1];
    for (i, &ai) in a.iter().enumerate() {
        pos[ai].push(i + 1);
    }

    for (l, r, x) in queries {
        let l = query(&pos[x], l);
        let r = query(&pos[x], r + 1);
        println!("{}", r - l);
    }
}
