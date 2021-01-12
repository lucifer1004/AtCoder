use proconio::input;

fn main() {
    input! {
        today: String,
    }

    let answer = match &*today {
        "Sunny" => "Cloudy",
        "Cloudy" => "Rainy",
        _ => "Sunny",
    };

    println!("{}", answer);
}
