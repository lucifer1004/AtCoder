use proconio::input;

const PRIMES: [usize; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }

    let mut ans = 0;
    for i in 1..1 << 15 {
        let mut num = 1;
        for j in 0..15 {
            if i & (1 << j) != 0 {
                num *= PRIMES[j];
            }
        }
        let mut ok = true;
        for j in 0..n {
            if gcd(x[j], num) == 1 {
                ok = false;
                break;
            }
        }
        if ok && (ans == 0 || ans > num) {
            ans = num;
        }
    }

    println!("{}", ans);
}