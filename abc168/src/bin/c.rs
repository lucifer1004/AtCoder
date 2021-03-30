use std::f64::consts::PI;

use proconio::input;

fn main() {
    input!{
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    };

    let h_angle = (h + m / 60.0) * 30.0;
    let m_angle = m * 6.0;
    let mut angle = (h_angle - m_angle).abs();
    if angle > 180.0 {
        angle = 360.0 - angle;
    }
    angle *= PI / 180.0;

    let ans = (a * a + b * b - 2.0 * a * b * angle.cos()).sqrt();

    println!("{}", ans);
}
