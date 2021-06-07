use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }

    let mut sum = 0;
    for &ti in t.iter() {
        sum += ti;
    }
    let mut target = sum / 2;
    let mut can = vec![false; target + 1];
    can[0] = true;
    for &ti in t.iter() {
        for i in (ti..=target).rev() {
            if can[i - ti] {
                can[i] = true;
            }
        }
    }
    for i in (0..=target).rev() {
        if can[i] {
            println!("{}", sum - i);
            return;
        }
    }
}
