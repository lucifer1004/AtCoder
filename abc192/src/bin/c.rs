use proconio::input;

fn f(n: usize) -> usize {
    let n = n.to_string();
    let mut s: Vec<&str> = n.split("").collect();
    let mut t = s.clone();
    s.sort();
    t.sort_by(|a, b| if a < b {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Less
    });
    let s = s.join("").parse::<usize>().unwrap();
    let t = t.join("").parse::<usize>().unwrap();
    t - s
}

fn main() {
    input! {
        mut n: usize,
        k: usize,
    }

    for _ in 0..k {
        n = f(n);
    }

    println!("{}", n);
}
