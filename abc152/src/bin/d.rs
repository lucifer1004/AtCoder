use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let s = n.to_string();
    let digits = s
        .chars()
        .map(|x| str::parse::<usize>(&x.to_string()).unwrap())
        .collect::<Vec<usize>>();
    let len = s.len();
    let mid = if len <= 2 {
        0
    } else {
        str::parse::<usize>(&s[1..len - 1]).unwrap()
    };
    let mut ans = 0;
    let mut cnt = vec![vec![0; 10]; 10];
    for i in 1..=9 {
        for j in 1..=9 {
            let mut mid_th = 1;
            for k in 1..=len {
                if k >= 3 {
                    mid_th *= 10;
                }
                if k == 1 {
                    if i == j && i <= n {
                        cnt[i][i] += 1;
                    }
                } else if k == len {
                    if i == digits[0] {
                        if j <= digits[len - 1] {
                            cnt[i][j] += mid + 1;
                        } else {
                            cnt[i][j] += mid;
                        }
                    } else if i < digits[0] {
                        cnt[i][j] += mid_th;
                    }
                } else {
                    cnt[i][j] += mid_th;
                }
            }
        }
    }

    for i in 1..=9 {
        for j in 1..=9 {
            ans += cnt[i][j] * cnt[j][i];
        }
    }

    println!("{}", ans);
}
