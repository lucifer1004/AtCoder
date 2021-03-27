use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        n: usize,
        x: f64,
        y: f64,
        xh: f64,
        yh: f64,
    }

    let xo = (x + xh) / 2.;
    let yo = (y + yh) / 2.;
    let x0 = x - xo;
    let y0 = y - yo;
    let theta = 2. * PI / n as f64;
    let x1 = theta.cos() * x0 - theta.sin() * y0;
    let y1 = theta.sin() * x0 + theta.cos() * y0;

    println!("{} {}", x1 + xo, y1 + yo);
}
