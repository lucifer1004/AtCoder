use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut valid = true;
    for (i, c) in s.chars().enumerate() {
        if (i % 2 == 0 && c == 'L') || (i % 2 == 1 && c == 'R') {
            valid = false;
        }
    }

    if valid {
        println!("Yes");
    } else {
        println!("No");
    }
}
