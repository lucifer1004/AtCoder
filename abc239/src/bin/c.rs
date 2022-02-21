use proconio::input;

fn main() {
    input! {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    }

    let points = [(-2, 1), (-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1)];
    for i in 0..8 {
        for j in 0..8 {
            if i != j && points[i].0 - points[j].0 == x1 - x2 && points[i].1 - points[j].1 == y1 - y2 {
                println!("Yes");
                std::process::exit(0);
            }
        }
    }

    println!("No");
}
