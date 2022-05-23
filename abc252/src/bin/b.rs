use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; k],
    }

    let mut hi = 0;
    for &ai in a.iter() {
        hi = hi.max(ai);
    }

    for i in b {
        if a[i - 1] == hi {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
