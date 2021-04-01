use proconio::input;

const K: usize = 64;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
        u: [usize; n],
        v: [usize; n],
    }

    let mut ans = vec![vec![0usize; n]; n];
    for k in 0..K {
        let msk = 1usize << k;
        let mut must0 = vec![vec![false; n]; n];
        let mut must1 = vec![vec![false; n]; n];
        for i in 0..n {
            if s[i] == 0 && (u[i] & msk > 0) {
                for j in 0..n {
                    must1[i][j] = true;
                }
            }
            if s[i] == 1 && (u[i] & msk == 0) {
                for j in 0..n {
                    must0[i][j] = true;
                }
            }
            if t[i] == 0 && (v[i] & msk > 0) {
                for j in 0..n {
                    must1[j][i] = true;
                }
            }
            if t[i] == 1 && (v[i] & msk == 0) {
                for j in 0..n {
                    must0[j][i] = true;
                }
            }
        }

        let mut rows = vec![0usize; n];
        let mut cols = vec![0usize; n];

        for i in 0..n {
            for j in 0..n {
                if must0[i][j] && must1[i][j] {
                    println!("-1");
                    std::process::exit(0);
                }
                if must1[i][j] {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }

        for i in 0..n {
            if s[i] == 1 && (u[i] & msk > 0) && rows[i] == 0 {
                let mut fixed = false;
                for j in 0..n {
                    if must0[i][j] {
                        continue;
                    }

                    if t[j] == 0 && (v[j] & msk == 0) && cols[j] == n - 1 {
                        continue;
                    }

                    must1[i][j] = true;
                    rows[i] += 1;
                    cols[j] += 1;
                    fixed = true;
                    break;
                }
                if !fixed {
                    println!("-1");
                    std::process::exit(0);
                }
            }

            if t[i] == 1 && (v[i] & msk > 0) && cols[i] == 0 {
                let mut fixed = false;
                for j in 0..n {
                    if must0[j][i] {
                        continue;
                    }

                    if s[j] == 0 && (u[j] & msk == 0) && rows[j] == n - 1 {
                        continue;
                    }

                    must1[j][i] = true;
                    rows[j] += 1;
                    cols[i] += 1;
                    fixed = true;
                    break;
                }

                if !fixed {
                    println!("-1");
                    std::process::exit(0);
                }
            }
        }

        for i in 0..n {
            if s[i] == 0 && (u[i] & msk == 0) && rows[i] == n {
                println!("-1");
                std::process::exit(0);
            }

            if t[i] == 0 && (v[i] & msk == 0) && cols[i] == n {
                println!("-1");
                std::process::exit(0);
            }
        }

        for i in 0..n {
            for j in 0..n {
                if must1[i][j] {
                    ans[i][j] ^= msk;
                }
            }
        }
    }

    for i in 0..n {
        println!("{}", ans[i].clone().into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}
