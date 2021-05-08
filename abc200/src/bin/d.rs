use proconio::input;

const K: usize = 200;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    for i in 0..n {
        a[i] %= K;
    }

    let mut modes: Vec<Vec<usize>> = vec![vec![]; K];
    let mut target = 0;
    let n = n.min(8);
    for i in 1..(1 << n) {
        let mut sum = 0;
        for j in 0..n {
            if i & (1 << j) > 0 {
                sum = (sum + a[j]) % K;
            }
        }
        modes[sum].push(i);
        if modes[sum].len() == 2 {
            target = sum;
            break;
        }
    }

    if target == 0 {
        println!("No");
    } else {
        println!("Yes");
        for i in 0..2 {
            print!("{} ", modes[target][i].count_ones());
            for j in 0..n {
                if modes[target][i] & (1 << j) > 0 {
                    print!("{} ", j + 1);
                }
            }
            print!("\n");
        }
    }
}
