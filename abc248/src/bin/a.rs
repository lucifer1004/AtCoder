use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut v = vec![false; 10];
    for c in s.chars() {
        v[c as u8 as usize - '0' as u8 as usize] = true;
    }

    for i in 0..10 {
        if !v[i] {
            println!("{}", i);
        }
    }
}
