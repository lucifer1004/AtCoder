use proconio::input;
use std::collections::HashMap;

fn main() {
    let mut ans = HashMap::new();
    ans.insert("SUN", 7);
    ans.insert("MON", 6);
    ans.insert("TUE", 5);
    ans.insert("WED", 4);
    ans.insert("THU", 3);
    ans.insert("FRI", 2);
    ans.insert("SAT", 1);

    input! {
        today: String,
    }

    println!("{}", ans.get::<str>(&today).unwrap());
}
