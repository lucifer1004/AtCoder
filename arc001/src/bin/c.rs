use proconio::input;
use proconio::marker::Chars;

fn dfs(b: &mut Vec<Vec<char>>, row: usize, mut cols: usize, mut diag: usize, mut idiag: usize) {
    if row == 8 {
        println!(
            "{}",
            b.into_iter()
                .map(|x| x
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(""))
                .collect::<Vec<String>>()
                .join("\n")
        );
        std::process::exit(0);
    } else {
        let mut q = 8;
        for col in 0..8 {
            if b[row][col] == 'Q' {
                if q != 8 {
                    return;
                }
                q = col;
            }
        }
        if q != 8 {
            if (1 << q) & cols != 0 || (1 << (row + q)) & diag != 0 || (1 << (8 + row - q)) & idiag != 0 {
                return;
            }
            cols ^= 1 << q;
            diag ^= 1 << (row + q);
            idiag ^= 1 << (8 + row - q);
            dfs(b, row + 1, cols, diag, idiag);
        } else {
            for col in 0..8 {
                if (1 << col) & cols != 0 || (1 << (row + col)) & diag != 0 || (1 << (8 + row - col)) & idiag != 0 {
                    continue;
                }
                cols ^= 1 << col;
                diag ^= 1 << (row + col);
                idiag ^= 1 << (8 + row - col);
                b[row][col] = 'Q';
                dfs(b, row + 1, cols, diag, idiag);
                b[row][col] = '.';
                cols ^= 1 << col;
                diag ^= 1 << (row + col);
                idiag ^= 1 << (8 + row - col);
            }
        }
    }
}

fn main() {
    input! {
        mut board: [Chars; 8],
    }

    dfs(&mut board, 0, 0, 0, 0);

    println!("No Answer");
}
