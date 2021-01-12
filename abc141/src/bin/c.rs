use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        q: usize,
        a: [usize; q],
    }

    let mut pts = vec![k; n];
    for i in a.iter() {
        pts[i - 1] += 1;
    }
    for i in pts.iter() {
        if *i > (q as i32) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
