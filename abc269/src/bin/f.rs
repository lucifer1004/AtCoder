use proconio::input;

const MOD: u128 = 998_244_353;

fn seq_sum(start: u128, mut end: u128, step: u128) -> (u128, u128) {
    if end < start {
        return (0, 0);
    }

    if (end - start) % step != 0 {
        end = start + (end - start) / step * step;
    }

    let items = (end - start) / step + 1;
    (items, (start + end) * items / 2)
}

fn main() {
    input! {
        _: u128,
        m: u128,
        q: usize,
        queries: [[u128; 4]; q],
    }

    for query in queries {
        let a = query[0];
        let b = query[1];
        let c = query[2];
        let d = query[3];

        let s1 = if (a + c) % 2 == 1 {
            a * m + c
        } else {
            (a - 1) * m + c
        };
        let e1 = (b - 1) * m + c;
        let (items1, sum1) = seq_sum(s1, e1, m * 2);
        let (_, mut ssum) = if items1 > 0 {
            seq_sum(sum1, sum1 + items1 * (d - c), items1 * 2)
        } else {
            (0, 0)
        };

        if d > c {
            let s2 = if (a + c + 1) % 2 == 1 {
                a * m + c + 1
            } else {
                (a - 1) * m + c + 1
            };
            let e2 = (b - 1) * m + c + 1;
            let (items2, sum2) = seq_sum(s2, e2, m * 2);
            let (_, ssum2) = if items2 > 0 {
                seq_sum(sum2, sum2 + items2 * (d - c - 1), items2 * 2)
            } else {
                (0, 0)
            };
            ssum += ssum2;
        }

        println!("{}", ssum % MOD);
    }
}
