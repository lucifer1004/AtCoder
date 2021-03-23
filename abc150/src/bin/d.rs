use proconio::input;

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    for i in 0..n {
        a[i] /= 2;
    }

    let mut l = 1;
    for i in 0..n {
        l = lcm(l, a[i]);
        if l > m {
            println!("0");
            std::process::exit(0);
        }
    }

    for i in 0..n {
        if l / a[i] % 2 == 0 {
            println!("0");
            std::process::exit(0);
        }
    }

    println!("{}", (m / l + 1) / 2);
}