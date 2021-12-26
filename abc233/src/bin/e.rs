use proconio::input;

fn main() {
    input! {
        a: String,
    }

    let mut a: Vec<usize> = a.chars().map(|x| (x as u8 - '0' as u8) as usize).collect::<Vec<_>>();
    let n = a.len();
    for i in 1..n {
        a[i] += a[i - 1];
    }

    for i in (1..n).rev() {
        a[i - 1] += a[i] / 10;
        a[i] %= 10;
    }

    println!("{}{}", a[0], &a[1..].iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(""));
}
