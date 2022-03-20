use proconio::input;

fn main() {
    input! {
        _: usize,
        t: String,
    }

    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut current_direction = 0;
    let mut cx = 0;
    let mut cy = 0;
    for ch in t.chars().into_iter() {
        match ch {
            'S' => {
                cx += directions[current_direction].0;
                cy += directions[current_direction].1;
            },
            'R' => current_direction = (current_direction + 1) % 4,
            _ => (),
        }
    }

    println!("{} {}", cx, cy);
}
