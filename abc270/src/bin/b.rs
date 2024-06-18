use proconio::input;

fn main() {
    input! {
        x: isize,
        y: isize,
        z: isize,
    }

    if (x > y && y > 0) || (x < y && y < 0) {
        // Need to pick up the hammer at z first
        // Check whether z is also blocked

        if (z > y && y > 0) || (z < y && y < 0) {
            println!("-1");
        } else {
            println!("{}", z.abs() + (z - y).abs() + (x - y).abs());
        }
    } else {
        println!("{}", x.abs());
    }
}
