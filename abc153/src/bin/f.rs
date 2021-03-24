use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: usize,
        mut monsters: [(usize, usize); n],
    }

    let mut ans = 0;
    monsters.sort();

    let mut bombs: VecDeque<(usize, usize)> = VecDeque::new();
    let mut bomb_count = 0;

    for (pos, health) in monsters {
        while !bombs.is_empty() && bombs.front().unwrap().0 + d < pos {
            bomb_count -= bombs.front().unwrap().1;
            bombs.pop_front();
        }

        if bomb_count * a < health {
            let need = (health - bomb_count * a - 1) / a + 1;
            bomb_count += need;
            ans += need;
            bombs.push_back((pos + d, need));
        }
    }

    println!("{}", ans);
}
