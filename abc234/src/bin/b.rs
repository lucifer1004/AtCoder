use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(i32, i32); n],
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let dx = points[i].0 - points[j].0;
            let dy = points[i].1 - points[j].1;
            ans = ans.max(dx * dx + dy * dy);
        }
    }

    println!("{}", (ans as f64).sqrt());
}
