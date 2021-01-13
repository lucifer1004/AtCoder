use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    }

    let v = a * a * b;
    let angle = if x * 2. >= v {
        let c = (v - x) * 2. / a / a;
        (c / a).atan()
    } else {
        let c = x * 2. / a / b;
        (b / c).atan()
    } / std::f64::consts::PI
        * 180.;

    println!("{}", angle);
}
