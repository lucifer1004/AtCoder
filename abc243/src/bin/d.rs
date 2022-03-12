use proconio::input;

const INF: usize = 2_000_000_000_000_000_000;

fn main() {
    input! {
        _: usize,
        mut x: usize,
        s: String,
    }

    let mut extra = 0;
    let s = s.chars().collect::<Vec<_>>();
    for c in s {
        if c == 'U' {
            if extra > 0 {
                extra -= 1;
            } else {
                x >>= 1;
            }
        } else if c == 'L' {
            if extra > 0 || (x << 1) > INF {
                extra += 1;
            } else {
                x <<= 1;
            }
        } else if extra > 0 || (x << 1) > INF {
            extra += 1;
        } else {
            x = (x << 1) + 1;
        }
    }

    println!("{}", x);
}
