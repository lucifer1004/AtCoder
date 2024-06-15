use proconio::input;

fn dist(x: (usize, usize), y: (usize, usize)) -> usize {
    x.0.max(y.0) - x.0.min(y.0) + x.1.max(y.1) - x.1.min(y.1)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    if k < n || (k - n) % 2 == 1 {
        println!("No");
        return;
    }

    let mut maze = vec![vec!['+'; 2 * m + 1]; 2 * n + 1];
    maze[0][2 * m - 1] = 'S';
    maze[2 * n][2 * m - 1] = 'G';
    for i in (1..=2 * n).step_by(2) {
        for j in (1..=2 * m).step_by(2) {
            maze[i][j] = 'o';
            if i + 1 < 2 * n {
                maze[i + 1][j] = '-';
            }
            if j + 1 < 2 * m {
                maze[i][j + 1] = '|';
            }
        }
    }

    // Build path
    let mut path: Vec<(usize, usize)> = vec![];
    path.push((0, m - 1));
    let mut x = 0;
    let mut y = m - 1;

    // Case 1: Simply go left and right
    if n % 2 == 0 || (n % 2 == 1 && k <= (n - 1) * m + 1) {
        while k - path.len() > dist((x, y), (n - 1, m - 1)) {
            if x % 2 == 0 {
                if y > 0 {
                    y -= 1;
                } else {
                    x += 1;
                }
            } else {
                if y < m - 1 {
                    y += 1;
                } else {
                    x += 1;
                }
            }
            path.push((x, y));
        }
        while path.len() < k {
            if x < n - 1 {
                x += 1;
            } else {
                y += 1;
            }
            path.push((x, y));
        }
    } else {
        while k - path.len() > dist((x, y), (n - 1, m - 1)) && x <= n - 3 {
            if x % 2 == 0 {
                if y > 0 {
                    y -= 1;
                } else {
                    x += 1;
                }
            } else {
                if y < m - 1 {
                    y += 1;
                } else {
                    x += 1;
                }
            }
            path.push((x, y));
        }

        while k - path.len() > dist((x, y), (n - 1, m - 1)) {
            if y % 2 == 0 {
                if x < n - 1 {
                    x += 1;
                } else {
                    y += 1;
                }
            } else {
                if x == n - 1 {
                    x -= 1;
                } else {
                    y += 1;
                }
            }
            path.push((x, y));
        }

        while path.len() < k {
            if y < m - 1 {
                y += 1;
            } else {
                x += 1;
            }
            path.push((x, y));
        }
    }

    for i in 0..k - 1 {
        let (x1, y1) = path[i];
        let (x2, y2) = path[i + 1];
        // Open the wall between (x1, y1) and (x2, y2)
        maze[x1 + x2 + 1][y1 + y2 + 1] = '.';
    }

    println!("Yes");
    for i in 0..=2 * n {
        println!("{}", maze[i].iter().collect::<String>());
    }
}
