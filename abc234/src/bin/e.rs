use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    let s = x.to_string().len() as i32;
    for k in s..=18 {
        for leading in 1..=9 {
            for step in -9..=9 {
                let last = leading + step * (k - 1);
                if last < 0 || last > 9 {
                    continue;
                }
                let mut num = 0i64;
                for i in 0..k {
                    num = num * 10 + (leading + i * step) as i64;
                }
                if num >= x {
                    println!("{}", num);
                    std::process::exit(0);
                }
            }
        }
    }
}
