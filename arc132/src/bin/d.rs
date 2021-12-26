use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    }

    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    let mut a = vec![0];
    let mut b = vec![0];
    for i in 0..n + m {
        a.push(a[i]);
        b.push(b[i]);
        if s[i] == '0' {
            a[i + 1] += 1;
        }
        if t[i] == '0' {
            b[i + 1] += 1;
        }
    }

    let mut ans = 0;

    let solve = |mut last: char| -> i32 {
        let mut beauty = 0;
        let mut zeros = if last == '0' { 1 } else { 0 };
        for i in 2..=n + m {
            let lo = a[i].min(b[i]);
            let hi = a[i].max(b[i]);
            if zeros + 1 >= lo && zeros + 1 <= hi && last == '0' {
                beauty += 1;
                zeros += 1;
            } else if zeros >= lo && zeros <= hi {
                if last == '1' {
                    beauty += 1;
                } else {
                    last = '1';
                }
            } else {
                last = '0';
                zeros += 1;
            }
        }
        beauty
    };

    if s[0] == '0' || t[0] == '0' {
        ans = ans.max(solve('0'));
    }

    if s[0] == '1' || t[0] == '1' {
        ans = ans.max(solve('1'));
    }

    println!("{}", ans);
}
