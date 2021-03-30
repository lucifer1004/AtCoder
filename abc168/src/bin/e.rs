use std::collections::{BTreeSet, BTreeMap};

use proconio::input;

const MOD: usize = 1_000_000_007;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn sig(x: i64) -> i64 {
    if x > 0 {
        1
    } else if x == 0 {
        0
    } else {
        -1
    }
}

fn norm(a: i64, b: i64) -> (i64, i64) {
    if a == 0 && b == 0 {
        (0, 0)
    } else if a == 0 {
        (0, sig(b))
    } else if b == 0 {
        (sig(a), 0)
    } else {
        let g = gcd(a.abs(), b.abs());
        let ag = a.abs() / g;
        let bg = b.abs() / g;
        if sig(a) == sig(b) {
            (ag, bg)
        } else {
            (ag, -bg)
        }
    }
}

fn main() {
    input! {
        n: usize,
        sardines: [(i64, i64); n],
    };

    let mut mp: BTreeMap<(i64, i64), usize> = BTreeMap::new();
    let mut a0 = 0usize;
    let mut b0 = 0usize;
    let mut ab0 = 0usize;

    for (a, b) in sardines {
        let (an, bn) = norm(a, b);
        if an == 0 && bn == 0 {
            ab0 += 1;
        } else if an == 0 {
            a0 += 1;
        } else if bn == 0 {
            b0 += 1;
        } else {
            *mp.entry((an, bn)).or_insert(0) += 1;
        }
    }

    let mut two = vec![1usize; n + 1];
    for i in 1..=n {
        two[i] = two[i - 1] * 2 % MOD;
    }

    let mut tot = (two[a0] + two[b0] - 1) % MOD;
    let mut vis: BTreeSet<(i64, i64)> = BTreeSet::new();

    for (an, bn) in mp.keys() {
        let an = *an;
        let bn = *bn;
        if vis.contains(&(an, bn)) {
            continue;
        }
        let comp = if bn > 0 {
            (bn, -an)
        } else {
            (-bn, an)
        };
        let left = *mp.get(&(an, bn)).unwrap();
        let right = *mp.get(&comp).unwrap_or(&0);
        tot = tot * (two[left] + two[right] + MOD - 1) % MOD;
        vis.insert(comp);
    }

    println!("{}", (tot + ab0 + MOD - 1) % MOD);
}
