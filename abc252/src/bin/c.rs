use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut hi = vec![vec![0; 10]; 10];
    for si in s {
        let t = si.chars().collect::<Vec<_>>();
        for i in 0..10 {
            let idx = (t[i] as u8 - b'0') as usize;
            hi[idx][i] += 1;
        }
    }

    let mut ans = 1_000_000_000;
    for i in 0..10 {
        let mut c = 0;
        for j in 0..10 {
            if hi[i][j] > 0 {
                c = c.max((hi[i][j] - 1) * 10 + j);
            }
        }
        ans = ans.min(c);
    }

    println!("{}", ans);
}
