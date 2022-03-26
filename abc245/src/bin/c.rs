use proconio::input;

fn main() {
    input!{
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
    };

    let mut possible = vec![a[0], b[0]];
    for i in 1..n {
        let mut new_possible = vec![];
        let mut can_a = false;
        let mut can_b = false;
        for &p in &possible {
            if (p - a[i]).abs() <= k {
                can_a = true;
            }
            if (p - b[i]).abs() <= k {
                can_b = true;
            }
        }
        if can_a {
            new_possible.push(a[i]);
        }
        if can_b {
            new_possible.push(b[i]);
        }
        possible = new_possible;
    }

    if !possible.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
