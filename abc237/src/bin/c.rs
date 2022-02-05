use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut la = 0;
    let mut ra = 0;
    for i in (0..n).rev() {
        if s[i] == 'a' {
            ra += 1;
        } else {
            break;
        }
    }
    for i in 0..n {
        if s[i] == 'a' {
            la += 1;
        } else {
            break;
        }
    }

    if la == n {
        println!("Yes");
    } else if la > ra {
        println!("No");
    } else {
        let mut l = la;
        let mut r = n - 1 - ra;
        while l < r {
            if s[l] != s[r] {
                println!("No");
                std::process::exit(0);
            }
            l += 1;
            r -= 1;
        }

        println!("Yes");
    }
}
