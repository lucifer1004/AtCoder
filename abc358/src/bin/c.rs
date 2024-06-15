use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
    }

    let state = s
        .iter()
        .map(|si| {
            si.chars()
                .enumerate()
                .map(|(i, c)| if c == 'o' { 1 << i } else { 0 })
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();

    let mut ans = n;
    for i in 0..1 << n {
        let mut count = 0;
        let mut check = 0;
        for j in 0..n {
            if i & 1 << j != 0 {
                count += 1;
                check |= state[j];
            }
        }
        if check.count_ones() == m as u32 {
            ans = ans.min(count);
        }
    }

    println!("{}", ans);
}
