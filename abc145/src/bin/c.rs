use proconio::input;

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt()
}

fn main() {
    input! {
        n: usize,
        points: [(f64, f64); n],
    }

    let mut tot = 0.0;
    for i in 0..n {
        for j in i + 1..n {
            tot += dist(points[i].0, points[i].1, points[j].0, points[j].1);
        }
    }

    println!("{}", tot * 2.0 / n as f64);
}
