use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    a.sort();

    // Use two pointers for inner count
    let count = |psum| -> usize {
        let mut sum = 0;
        let mut l = 0;
        for r in (0..n).rev() {
            while l < r && a[l] + a[r] < psum {
                l += 1;
            }
            if r > l {
                sum += (r - l) * 2;
            }
            if a[r] * 2 >= psum {
                sum += 1;
            }
        }
        sum
    };

    // Binary search for the highest lower limit of pair sum such that there are at least M pairs
    let mut lo: usize = 2;
    let mut hi: usize = 200_000;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if count(mid) < m {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    let larger = count(hi + 1);
    let threshold_sum = (m - larger) * hi;
    let mut larger_sum = 0;
    let mut suffix_sum = vec![0; n + 1];
    for i in (0..n).rev() {
        suffix_sum[i] = suffix_sum[i + 1] + a[i];
    }
    for ai in a.clone() {
        let mut inner_lo: usize = 0;
        let mut inner_hi: usize = n - 1;
        while inner_lo <= inner_hi {
            let inner_mid = (inner_lo + inner_hi) / 2;
            if a[inner_mid] + ai <= hi {
                inner_lo = inner_mid + 1;
            } else {
                if inner_mid == 0 {
                    break;
                }
                inner_hi = inner_mid - 1;
            }
        }
        larger_sum += (n - inner_lo) * ai + suffix_sum[inner_lo];
    }

    let ans = larger_sum + threshold_sum;
    println!("{}", ans);
}
