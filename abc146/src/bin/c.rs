use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64,
    }

    let max_digits = (x / b).min(18);
    for d in (1..=max_digits).rev() {
        let rem = x - d * b;
        let n = rem / a;
        if n.to_string().len() >= d as usize {
            println!(
                "{}",
                if d >= 10 {
                    1_000_000_000i64
                } else {
                    n.min(10i64.pow(d as u32) - 1)
                }
            );
            return;
        }
    }

    println!("0");
}
