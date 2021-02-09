use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
    }

    let mut testimonies: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for i in 0..n {
        input! {
            a: usize,
        }
        for _ in 0..a {
            input! {
                x: Usize1,
                y: usize,
            }
            testimonies[i].push((x, y));
        }
    }

    let mut ans = 0;
    for i in 0..1 << n {
        let mut ok = true;
        let mut cnt = 0;
        let mut determined = vec![0; n];
        for j in 0..n {
            if i & (1 << j) > 0 {
                cnt += 1;
                for (x, y) in testimonies[j].clone() {
                    if (determined[x] > 0 && determined[x] != y + 1)
                        || (y == 0 && (i & (1 << x) > 0))
                        || (y == 1 && (i & (1 << x) == 0))
                    {
                        ok = false;
                        break;
                    }
                    determined[x] = y + 1;
                }
            }
            if !ok {
                break;
            }
        }
        if ok {
            ans = ans.max(cnt);
        }
    }

    println!("{}", ans);
}
