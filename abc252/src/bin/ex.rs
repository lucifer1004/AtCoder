use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        c: usize,
        mut k: usize,
        items: [(Usize1, usize); n],
    }

    let mut a: Vec<usize> = vec![0];
    for cc in 0..=c / 2 {
        let mut na= vec![];
        for &(ci, vi) in items.iter() {
            if ci == cc {
                for &ai in a.iter() {
                    na.push(ai ^ vi);
                }
            }
        }
        a = na;
    }

    let mut b: Vec<usize> = vec![0];
    for cc in c / 2 + 1..c {
        let mut nb = vec![];
        for &(ci, vi) in items.iter() {
            if ci == cc {
                for &bi in b.iter() {
                    nb.push(bi ^ vi);
                }
            }
        }
        b = nb;
    }

    let mut ans: usize = 0;
    let mut v: Vec<(Vec<usize>, Vec<usize>)> = vec![(a, b)];
    for i in (0..60usize).rev() {
        let mut cnt: usize = 0;
        let mut arr: Vec<([Vec<usize>; 2], [Vec<usize>; 2])> = vec![];
        for (a, b) in v {
            let mut na: [Vec<usize>; 2] = [vec![], vec![]];
            let mut nb: [Vec<usize>; 2] = [vec![], vec![]];
            for &ai in a.iter() {
                na[(ai >> i) & 1].push(ai);
            }
            for &bi in b.iter() {
                nb[(bi >> i) & 1].push(bi);
            }
            cnt += na[0].len() * nb[1].len() + na[1].len() * nb[0].len();
            arr.push((na, nb));
        }

        let mut nxt: Vec<(Vec<usize>, Vec<usize>)> = vec![];
        if cnt < k {
            k -= cnt;
            for (a, b) in arr {
                if !a[0].is_empty() && !b[0].is_empty() {
                    nxt.push((a[0].clone(), b[0].clone()));
                }
                if !a[1].is_empty() && !b[1].is_empty() {
                    nxt.push((a[1].clone(), b[1].clone()));
                }
            }
        } else {
            ans |= 1usize << i;
            for (a, b) in arr {
                if !a[0].is_empty() && !b[1].is_empty() {
                    nxt.push((a[0].clone(), b[1].clone()));
                }
                if !a[1].is_empty() && !b[0].is_empty() {
                    nxt.push((a[1].clone(), b[0].clone()));
                }
            }
        }

        v = nxt;
    }

    println!("{}", ans);
}