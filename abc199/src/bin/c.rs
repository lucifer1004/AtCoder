use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
        q: usize,
        ops: [(usize, usize, usize); q],
    }

    let mut swapped = false;
    let mut c = s.chars().collect::<Vec<char>>();
    for (t, a, b) in ops {
        if t == 1 {
            let mut a = a - 1;
            let mut b = b - 1;
            if swapped {
                a = (a + n) % (n * 2);
                b = (b + n) % (n * 2);
            }
            let tmp = c[a];
            c[a] = c[b];
            c[b] = tmp;
        } else {
            swapped = !swapped;
        }
    }

    if swapped {
        for i in 0..n {
            let tmp = c[i];
            c[i] = c[n + i];
            c[n + i] = tmp;
        }
    }

    println!("{}", c.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""))
}
