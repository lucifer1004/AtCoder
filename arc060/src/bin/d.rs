use proconio::input;

fn calc(mut n: usize, b: usize) -> usize {
    if b == 1 {
        return 0;
    }
    let mut ans = 0;
    while n > 0 {
        ans += n % b;
        n /= b;
    }
    ans
}

fn main() {
    input! {
        n: usize,
        s: usize,
    }

    if n < s {
        println!("-1");
    } else if n == s {
        println!("{}", n + 1);
    } else {
        let mut b = 1;
        let mut ans = n;
        while b * b <= n {
            if calc(n, b) == s {
                println!("{}", b);
                std::process::exit(0);
            }
            if (n - s) % b == 0 && calc(n, (n - s) / b + 1) == s {
                ans = ans.min((n - s) / b + 1);
            }
            b += 1;
        }
        if ans == n && s != 1 {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}
