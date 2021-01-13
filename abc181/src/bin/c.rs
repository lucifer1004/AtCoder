use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut points: Vec<(i32, i32)> = vec![];

    for _ in 0..n {
        input! {
            x: i32,
            y: i32,
        }
        points.push((x, y));
    }

    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            let dij = (points[i].0 - points[j].0, points[i].1 - points[j].1);
            for k in j + 1..n {
                let dik = (points[i].0 - points[k].0, points[i].1 - points[k].1);
                if dij.0 * dik.1 == dij.1 * dik.0 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
