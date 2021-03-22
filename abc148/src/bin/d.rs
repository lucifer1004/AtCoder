use proconio::input;

fn main() {
    input! {
        n: usize,
        bricks: [i32; n],
    }

    let mut useful = 0;
    for brick in bricks {
        if brick == useful + 1 {
            useful += 1;
        }
    }

    println!("{}", if useful > 0 { n as i32 - useful } else { -1 });
}
