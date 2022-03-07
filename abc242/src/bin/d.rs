use proconio::input;

fn main() {
    input! {
        s: String,
        q: usize,
        queries: [(usize, usize); q],
    }

    let s: Vec<usize> = s.chars()
        .map(|x| if x == 'A' { 0 } else if x == 'B' { 1 } else { 2 })
        .collect::<Vec<_>>();

    for (t, k) in queries {
        let mut pos = k - 1;
        let mut round = t;
        let mut ops = vec![];
        while pos > 0 && round > 0 {
            ops.push(pos & 1);
            pos >>= 1;
            round -= 1;
        }

        let mut char = s[pos];
        if round != 0 {
            char = (char + round) % 3;
        }

        ops.reverse();
        for op in ops {
            char = (char + op + 1) % 3;
        }

        println!("{}", if char == 0 { "A" } else if char == 1 { "B" } else { "C" });
    }
}
