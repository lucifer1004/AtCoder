use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut lo = 0;
    let mut hi = k;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        let tot = a.iter().map(|&x| x.min(mid)).sum::<usize>();

        if tot >= k {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    let consumed = a.iter().map(|&x| x.min(hi)).sum::<usize>();
    let mut remaining = k - consumed;
    let mut b = a
        .iter()
        .map(|&x| if x >= hi { x - hi } else { 0 })
        .collect::<Vec<usize>>();

    let mut i = 0;
    while remaining > 0 {
        if b[i] > 0 {
            b[i] -= 1;
            remaining -= 1;
        }
        i += 1;
        if i == n {
            i = 0;
        }
    }

    println!(
        "{}",
        b.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
