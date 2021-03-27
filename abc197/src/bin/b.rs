use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        y: usize,
        grid: [String; h],
    }

    let grid = grid
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut ans = 1;
    for i in x + 1..=h {
        if grid[i - 1][y - 1] == '#' {
            break;
        }
        ans += 1;
    }
    for i in (1..x).rev() {
        if grid[i - 1][y - 1] == '#' {
            break;
        }
        ans += 1;
    }
    for j in y + 1..=w {
        if grid[x - 1][j - 1] == '#' {
            break;
        }
        ans += 1;
    }
    for j in (1..y).rev() {
        if grid[x - 1][j - 1] == '#' {
            break;
        }
        ans += 1;
    }

    println!("{}", ans);
}
