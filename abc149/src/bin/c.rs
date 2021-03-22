use proconio::input;

fn is_prime(x: usize) -> bool {
    if x == 1 {
        false
    } else {
        let mut i = 2;
        while i * i <= x {
            if x % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }
}

fn main() {
    input! {
        mut x: usize,
    }

    while !is_prime(x) {
        x += 1;
    }

    println!("{}", x);
}
