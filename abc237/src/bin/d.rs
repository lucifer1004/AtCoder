use proconio::input;

const INF: usize = 1_000_000_000;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut right = vec![INF; n + 2];
    let mut left = vec![INF; n + 2];
    right[0] = 1;
    left[1] = 0;

    let mut i = 1;
    for c in s.chars() {
        i += 1;

        if c == 'L' {
            right[left[i - 1]] = i;
            left[i] = left[i - 1];
            left[i - 1] = i;
            right[i] = i - 1;
        } else {
            if right[i - 1] != INF {
                left[right[i - 1]] = i;
                right[i] = right[i - 1];
            }

            right[i - 1] = i;
            left[i] = i - 1;
        }
    }

    let mut p = 0;
    for _ in 0..=n {
        p = right[p];
        print!("{} ", p - 1);
    }
}
