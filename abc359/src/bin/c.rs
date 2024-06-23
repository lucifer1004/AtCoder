use proconio::input;

fn main() {
    input! {
        sx: isize,
        sy: isize,
        tx: isize,
        ty: isize,
    }

    let dx = (tx - sx).abs();
    let dy = (ty - sy).abs();

    let xc = if (sx + sy) % 2 == 0 {
        if tx >= sx {
            if dx <= dy + 1 {
                0
            } else {
                (dx - dy) / 2
            }
        } else {
            if dx <= dy {
                0
            } else {
                (dx - dy + 1) / 2
            }
        }
    } else {
        if tx > sx {
            if dx <= dy {
                0
            } else {
                (dx - dy + 1) / 2
            }
        } else {
            if dx <= dy + 1 {
                0
            } else {
                (dx - dy) / 2
            }
        }
    };

    println!("{}", xc + dy);
}
